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

static struct dentry *dir;
static const char *root = "rfaulty";

static int init_endpoint(struct dentry *dir, const char *fn, const struct file_operations *fops);

static const struct file_operations fops_sbo = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_stack_read,
	.write = rust_stack_write,
};

static const struct file_operations fops_slab = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_slab_read,
	.write = rust_slab_write,
};

static const struct file_operations fops_overflow = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_unsigned_overflow_read,
};

static const struct file_operations fops_underflow = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_signed_underflow_read,
};

static const struct file_operations fops_format = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_format_read,
	.write = rust_format_write,
};

static const struct file_operations fops_race = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_race_read,
	.write = rust_race_write,
};

static const struct file_operations fops_double_free = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_df_alloc,
	.write = rust_df_free,
};

// use after free
static const struct file_operations fops_use_after_free = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_use_after_free_read,
};

static const struct file_operations fops_infoleak = {
	.owner = THIS_MODULE,
	.open = simple_open,
	.read = rust_infoleak_read,
};

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
		race_init();
		pr_debug("Rust-Faulty: Data race at debugfs '%s/data-race'\n", root);
	}

	if (!init_endpoint(dir, "double-free", &fops_double_free))
		pr_debug("Rust-Faulty: Double free bug at debugfs '%s/double-free'\n", root);

	if (!init_endpoint(dir, "use-after-free", &fops_use_after_free))
		pr_debug("Rust-Faulty: Double free bug at debugfs '%s/use-after-free'\n", root);

	infoleak_init();
	if (!init_endpoint(dir, "infoleak", &fops_infoleak))
		pr_debug("Rust-Faulty: Infoleak at debugfs '%s/infoleak'\n", root);

	pr_debug("Rust-Faulty: module loaded\n");
	return 0;

}

static void __exit mod_exit(void)
{
	debugfs_remove_recursive(dir);
	race_exit();
	infoleak_exit();

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

module_init(mod_init);
module_exit(mod_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("A Kernel Module with Faults, Rust version");
MODULE_AUTHOR("Ilja Sidoroff, Based on the work of Taesoo Kim");
