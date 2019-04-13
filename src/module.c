/*
 * RFaulty: A translation of faulty C kernel module into Rust
 */
#include <linux/bug.h>
#include <linux/debugfs.h>
#include <linux/delay.h>
#include <linux/fs.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/uaccess.h>
#include <linux/slab.h>

#include "rust-impl.h"

char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

void abort(void)
{
    BUG();
}

#define BUF_SIZE 256

static struct dentry *dir;
static const char *root = "rfaulty";

static int init_endpoint(struct dentry *dir, const char *fn, const struct file_operations *fops);
static ssize_t signed_underflow_read(struct file *fps, char __user *buf, size_t len, loff_t *offset);
static ssize_t race_read(struct file *fps, char __user *buf, size_t len, loff_t *offset);
static ssize_t race_write(struct file *fps, const char __user *buf, size_t len, loff_t *offset);
static ssize_t df_alloc(struct file *fps, char __user *buf, size_t len, loff_t *offset);
static ssize_t df_free(struct file *fps, const char __user *buf, size_t len, loff_t *offset);
static ssize_t use_after_free_read(struct file *fps, char __user *buf, size_t len, loff_t *offset);
static ssize_t infoleak_read(struct file *fps, char __user *buf, size_t len, loff_t *offset);
static void non_reachable_function(void);

static char *buffer = "just some small data buffer\n";

static const struct file_operations fops_sbo = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_stack_read,
	.write = rust_stack_write,
};

// slab corruption
static const struct file_operations fops_slab = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_slab_read,
	.write = rust_slab_write,
};

// under/overflow
static s8 signed_counter = -124;

static const struct file_operations fops_overflow = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_unsigned_overflow_read,
};

static const struct file_operations fops_underflow = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = signed_underflow_read,
};

static const struct file_operations fops_format = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_format_read,
	.write = rust_format_write,
};

// data race
static char *race1;
static char *race2;

static const struct file_operations fops_race = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = race_read,
	.write = race_write,
};

// double free
static char *double_free;

static const struct file_operations fops_double_free = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = df_alloc,
	.write = df_free,
};

// use after free
static const struct file_operations fops_use_after_free = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = use_after_free_read,
};

static const struct file_operations fops_infoleak = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = infoleak_read,
};

// FAULT: infoleak
#define DATA_LEN 4096
struct a_struct {
	char data[DATA_LEN];
} *uninitialized;

static int __init mod_init(void)
{
	pr_debug("Rust-Faulty: creating debugfs-endpoints\n");

	dir = debugfs_create_dir(root, NULL);

	if (dir == ERR_PTR(-ENODEV)) {
		pr_err
		    ("Rust-Faulty: Debugfs doesn't seem to be compiled into the kernel\n");
		return -ENODEV;
	}

	if (dir == NULL) {
		pr_err
		    ("Rust-Faulty: Cannot create debugfs-entry '%s'", root);
		return -ENOENT;
	}

	if (!init_endpoint(dir, "sbo", &fops_sbo))
		pr_debug
		    ("Rust-Faulty: Stack buffer overflow at debugfs '%s/sbo'\n", root);

	if (!init_endpoint(dir, "slab", &fops_slab))
		pr_debug("Rust-Faulty: Slab buffer overflow at debugfs '%s/slab'\n", root);

	if (!init_endpoint(dir, "overflow", &fops_overflow))
		pr_debug("Rust-Faulty: Unsigned integer overflow at debugfs '%s/overflow'\n", root);

	if (!init_endpoint(dir, "underflow", &fops_underflow))
		pr_debug("Rust-Faulty: Signed integer underflow at debugfs '%s/underflow'\n", root);

	if (!init_endpoint(dir, "format", &fops_format))
		pr_debug("Rust-Faulty: Format string bug at debugfs '%s/format'\n", root);

	if (!init_endpoint(dir, "data-race", &fops_race)) {
		race1 = kzalloc(PAGE_SIZE, GFP_KERNEL);
		if (!race1) {
			pr_debug("Rust-Faulty: Race - cannot allocate buffer 1\n");
			goto end;
		}
		race2 = kzalloc(PAGE_SIZE, GFP_KERNEL);
		if (!race2) {
			pr_debug("Rust-Faulty: Race - cannot allocate buffer 2\n");
			kfree(race1);
			goto end;
		}
		pr_debug("Rust-Faulty: Format string bug at debugfs '%s/data-race'\n", root);
	}

	if (!init_endpoint(dir, "double-free", &fops_double_free))
		pr_debug("Rust-Faulty: Double free bug at debugfs '%s/double-free'\n", root);

	if (!init_endpoint(dir, "use-after-free", &fops_use_after_free))
		pr_debug("Rust-Faulty: Double free bug at debugfs '%s/use-after-free'\n", root);

	uninitialized = kmalloc(sizeof (struct a_struct), GFP_KERNEL);
	if (!uninitialized) {
		pr_debug("Rust-Faulty: Infoleak - cannot allocate buffer\n");
		goto end;
	}

	if (!init_endpoint(dir, "infoleak", &fops_infoleak))
		pr_debug("Rust-Faulty: Infoleak at debugfs '%s/infoleak'\n", root);

end:
	pr_debug("Rust-Faulty: module loaded\n");
	return 0;

}

static void __exit mod_exit(void)
{
	debugfs_remove_recursive(dir);
	kfree(race1);
	kfree(race2);
	kfree(uninitialized);

	pr_debug("Rust-Faulty: Unloaded faulty kernel module\n");
}

static int init_endpoint(struct dentry *dir, const char *fn, const struct file_operations *fops)
{
	struct dentry *fil = debugfs_create_file(fn, 0644, dir, NULL, fops);

	if (fil == NULL) {
		pr_err("Rust-Faulty: Cannot create endpoint %s\n", fn);
		return -ENOENT;
	}

	return 0;
}

static ssize_t signed_underflow_read(struct file *fps, char __user *buf,
				     size_t len, loff_t *offset)
{
	char *buffer = kmalloc(BUF_SIZE, GFP_KERNEL);
	ssize_t n = 0;

	// FAULT: signed underflow
	snprintf(buffer, BUF_SIZE, "Rust-Faulty: Underflow - Counter value :%d\n",
		signed_counter--); // note the behaviour of counter

	if (signed_counter == 126)
		non_reachable_function();

	n =  simple_read_from_buffer(buf, len, offset, buffer,
				       strlen(buffer));
	kfree(buffer);
	return n;
}

static ssize_t race_read(struct file *fps, char __user *buf, size_t len,
			loff_t *offset)
{
	if (strcmp(race1, race2)) {
		non_reachable_function();
	}
	return simple_read_from_buffer(buf, len, offset, race1,
				strlen(race1));
}

static ssize_t race_write(struct file *fps, const char __user *buf, size_t len,
			 loff_t *offset)
{
	// FAULT: stack overflow
	char buffer[PAGE_SIZE];
	ssize_t n;

	n = simple_write_to_buffer(&buffer, PAGE_SIZE, offset, buf, len);
	buffer[n] = '\0';

	// FAULT: race
	// slow write is racy
	memcpy(race1, buffer, len);
	udelay(100);
	memcpy(race2, buffer, len);

	return n;
}

static ssize_t df_alloc(struct file *fps, char __user *buf, size_t len, loff_t *offset)
{
	double_free = kmalloc(len, GFP_KERNEL);
	return len;
}
static ssize_t df_free(struct file *fps, const char __user *buf, size_t len, loff_t *offset)
{
	// FAULT: double free
	kfree(double_free);
	return len;
}

static ssize_t use_after_free_read(struct file *fps, char __user *buf, size_t len, loff_t *offset)
{
	char *tmp = kmalloc(len, GFP_KERNEL);
	strncpy(tmp, buffer, len);
	// FAULT: use after free
	kfree(tmp);
	copy_to_user(buf, tmp, len);
	return len;
}


static ssize_t infoleak_read(struct file *fps, char __user *buf, size_t len, loff_t *offset)
{

	ssize_t l = len < DATA_LEN ? len : DATA_LEN;
	return simple_read_from_buffer(buf, len, offset, uninitialized->data,
				       l);

}

static void non_reachable_function(void)
{
	pr_info("Rust-Faulty: This function should not be reachable.\n");
}

module_init(mod_init);
module_exit(mod_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("A Kernel Module with Faults, Rust version");
MODULE_AUTHOR("Ilja Sidoroff, Based on the work of Taesoo Kim");
