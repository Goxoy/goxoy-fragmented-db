# Goxoy Fragmented DB

Büyük veri kaydetmek için SQLite veri tabanını Key-Value olarak parçalayıp kaydeden kitaplık


## Kullanım / Örnekler

```rust
let mut fdb_obj = Fragmented::new();

// kayıt ekleme ve güncelleme için
fdb_obj.set_value("key-text", "value-text");

// kayıt silmek için
fdb_obj.delete("key-text");

// kayıt okumak için
let value_obj : Option<String> = fdb_obj.get_value("key-text");
if value_obj.is_some(){
    println!("bulunan kayit: {}", value_obj.unwrap());
}else{
    println!("kayit bulunamadi");
}

```

  
## Lisans

[MIT](https://choosealicense.com/licenses/mit/)