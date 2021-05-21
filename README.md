# isitup
[Rust](https://www.rust-lang.org/)y CLI utility to check if a website/domain is up or down, leveraging [isitup](https://isitup.org/) API

## Usage
- Pass a space separated list of domains
```bash
$ ./isitup google.com github.com blah.com blah

google.com is up! 🎉
It took 0.076 seconds time to get a 200 status code from an IP address of 142.250.179.196.

github.com is up! 🎉
It took 0.05 seconds time to get a 200 status code from an IP address of 140.82.121.4.

blah.com seems to be down! 💥

blah does not seem to be a valid domain. ❌🌐
```
- Or, run without arguments to get a prompt
```bash
$ ./isitup
Enter domains, space separated: google.com github.com

google.com is up! 🎉
It took 0.092 seconds time to get a 200 status code from an IP address of 142.250.179.132.

github.com is up! 🎉
It took 0.047 seconds time to get a 200 status code from an IP address of 140.82.121.3.

Enter domains, space separated: blah.com blah

blah.com seems to be down! 💥

blah does not seem to be a valid domain. ❌🌐

Enter domains, space separated: ^C
```