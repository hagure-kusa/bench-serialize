Rustで使えるバイナリシリアライザのパフォーマンス計測

## 使い方

```
$ cargo run --release
with serde_bytes:
bincode   (encode) 4.8973ms
bincode   (decode) 2.3063ms
cbor      (encode) 4.2206ms
cbor      (decode) 2.7095ms
rmp serde (encode) 3.8015ms
rmp serde (decode) 2.4234ms

without serde_bytes:
bincode   (encode) 70.3723ms
bincode   (decode) 71.2949ms
cbor      (encode) 105.8754ms
cbor      (decode) 89.4747ms
rmp serde (encode) 133.3205ms
rmp serde (decode) 87.1423ms
```
