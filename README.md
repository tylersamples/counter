# Rustler Resource Objects Counter Example

Setup:
```
$ git clone git@github.com:tylersamples/counter.git && cd counter
$ mix deps.get
```

Using the counter:
```
iex(1)> Counter.new()
#Reference<0.3379365608.2720137221.242534>
iex(2)> counter = Counter.new()
#Reference<0.4020670021.1591345154.158521>
iex(3)> Counter.__info__(:functions)
[__init__: 0, decrement: 1, increment: 1, new: 0, read: 1]
iex(4)> counter |> Counter.increment() |> Counter.decrement() |> Counter.read()
0
```

More information can be found: https://smpls.dev/p/rustler-resources

