use std::{collections::HashMap, fs, time::{SystemTime, UNIX_EPOCH}, thread};

use goxoy_key_value::key_value::KeyValueDb;

#[derive(Debug)]
pub struct Fragmented {
    list:HashMap<String,String>,
    path:String
}

impl Fragmented {
    pub fn new() -> Self {
        let common_path=String::from("./db/");
        _ = fs::create_dir_all(&common_path.clone());
        
        let mut tmp_dir=common_path.clone();
        tmp_dir.push_str("tmp/");
        _ = fs::create_dir_all(&tmp_dir.clone());
        
        let paths = fs::read_dir(tmp_dir);
        if paths.is_ok(){
            let paths=paths.unwrap();
            for path in paths {
                if path.is_ok(){
                    println!("Name: {}", path.unwrap().path().display())
                }else{
                    println!("ERR : {}", path.unwrap().path().display())
                }
            }        
        }

        let mut temp_db_name=common_path.clone();
        temp_db_name.push_str("temp_data");

        Fragmented{
            list:HashMap::new(),
            path:common_path.clone(),
        }
    }
    pub fn set_value(&mut self,key:&str,value:&str) -> bool {
        self.list.insert(String::from(key.clone()), String::from(value.clone()));

        let time_no=SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos();
        let mut tmp_file_path=self.path.clone();
        tmp_file_path.push_str("tmp/");
        tmp_file_path.push_str(&time_no.clone().to_string());
        tmp_file_path.push_str(".data");

        let mut full_text=hex::encode(key.clone());
        full_text.push_str("||");
        full_text.push_str(&hex::encode(value.clone()));

        let fs_result=fs::write(tmp_file_path.clone(), full_text);
        if fs_result.is_ok(){
            let key_tmp=String::from(key);
            let value_tmp=String::from(value);
            let db_path_tmp=self.path.clone();
            let tmp_file_cloned=tmp_file_path.clone();
            thread::spawn(move ||{
                Fragmented::set_to_sub_db(&key_tmp,&value_tmp,db_path_tmp,tmp_file_cloned);
            });
            return true;
        }
        false
    }
    pub fn get_value(&mut self,key:&str)->String{
        if self.list.contains_key(key){
            let result=self.list.get(key);
            if result.is_some(){
                return String::from(result.unwrap());
            }
        }
        let path_tmp=Fragmented::get_key_db_name(key,self.path.clone());
        let mut tmp_db_obj=KeyValueDb::new(&path_tmp);
        let result_val=tmp_db_obj.get_value(&key);
        tmp_db_obj.close();
        if result_val.is_some(){
            return String::from(result_val.unwrap());
        }
        return String::from("");
    }
    pub fn get_key_db_name(key:&str,path_val:String)->String{
        let hash_result=goxoy_hash::hash::Hash::calculate(goxoy_hash::hash::HashKind::SHA1, &key.clone());
        let first_2_letter=&hash_result[..2];
        let mut path_tmp=path_val.clone();
        path_tmp.push_str("data_");
        path_tmp.push_str(first_2_letter);
        return path_tmp;
    }
    pub fn set_to_sub_db(key:&str,value:&str,path_val:String,tmp_file:String){
        let path_tmp=Fragmented::get_key_db_name(key,path_val);
        let mut tmp_db_obj=KeyValueDb::new(&path_tmp);
        tmp_db_obj.set_value(&key, &value);
        tmp_db_obj.close();
        _ = fs::remove_file(&tmp_file);
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut kv_obj=Fragmented::new();
    kv_obj.set_value("key-degeri-1","value-degeri-1");
    kv_obj.set_value("key-degeri-2","value-degeri-2");
    kv_obj.set_value("key-degeri-3","value-degeri-3");
    for _ in 0..1_000_000{
        for _ in 0..100{
            
        }
    }
    let dd=kv_obj.get_value("key-degeri-3");
    dbg!(dd);
    let dd=kv_obj.get_value("key-degeri-5");
    dbg!(dd);
    assert!(true)
}
