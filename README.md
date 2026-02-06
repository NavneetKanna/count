## Count

This is a command line tool to count the number of files in the current directory.
You might wonder what is the need of this tool when we can do either of the following

```bash
ls -1 | wc -l
gdu --inodes
du --inodes
```

Well the answer is simple:

- This is a good project to start to learn rust.
- The commands are intuitive for counting the number of files. Sure we can create alias, but these 
  commands are not exclusively meant for counting.
- Better output formatting.
