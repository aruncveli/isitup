# isitup
[Rust](https://www.rust-lang.org/)y CLI utility to check if a website/domain is up or down, leveraging [isitup](https://isitup.org/) API

## Usage
Pass a space separated list of domains as arguments
```bash
$ ./isitup blah blah.com google.com
  Can access isitup.org! ✅

  blah does not seem to be a valid domain! ❌
  blah.com seems to be down! 💥
  google.com is up! 🎉
```