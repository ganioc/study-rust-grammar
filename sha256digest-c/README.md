

```
gcc -c main.c -o main.o


Undefined symbols for architecture x86_64:
  "_SecRandomCopyBytes", referenced from:
      ring::rand::darwin::fill::h05db493ab8c95d70 in libsha256digest.a(ring-b364441b690091a4.ring.a46fa014-cgu.2.rcgu.o)
  "_kSecRandomDefault", referenced from:
      ring::rand::darwin::fill::h05db493ab8c95d70 in libsha256digest.a(ring-b364441b690091a4.ring.a46fa014-cgu.2.rcgu.o)
ld: symbol(s) not found for architecture x86_64

```

说明链接 rust编译出的库的时候，还需要包含rust package的库才行。为什么没有直接打包在一起呢？


