
extern ssize_t rust_format_read(struct file *fps, char *buf, size_t len,
				loff_t *offset);
extern ssize_t rust_format_write(struct file *fps, const char __user *buf,
				size_t len, loff_t *offset);

extern ssize_t rust_stack_read(struct file *fps, char *buf, size_t len,
			loff_t *offset);
extern ssize_t rust_stack_write(struct file *fps, const char __user *buf,
			       size_t len, loff_t *offset);

extern ssize_t rust_slab_read(struct file *fps, char *buf, size_t len,
			loff_t *offset);
extern ssize_t rust_slab_write(struct file *fps, const char __user *buf,
			size_t len, loff_t *offset);

extern ssize_t rust_unsigned_overflow_read(struct file *fps, char *buf,
					size_t len, loff_t *offset);

extern ssize_t rust_signed_underflow_read(struct file *fps, char *buf,
					size_t len, loff_t *offset);

extern void init_race(void);
extern void exit_race(void);

extern ssize_t rust_race_read(struct file *fps, char *buf,
			size_t len, loff_t *offset);

extern ssize_t rust_race_write(struct file *fps, const char __user *buf,
			size_t len, loff_t *offset);


extern ssize_t rust_df_alloc(struct file *fps, char *buf,
			size_t len, loff_t *offset);

extern ssize_t rust_df_free(struct file *fps, const char __user *buf,
			size_t len, loff_t *offset);

extern ssize_t rust_use_after_free_read(struct file *fps, char *buf,
					size_t len, loff_t *offset);
