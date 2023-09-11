use std::{collections::HashMap, fs, time::{SystemTime, UNIX_EPOCH}, thread};
use goxoy_key_value::key_value::KeyValueDb;
#[derive(Debug)]
pub struct Fragmented {
    fragmented_db_count:u16,
    list:HashMap<String,String>,
    path:String
}

impl Fragmented {
    pub fn new(fragmented_db_count:u16) -> Self {
        let common_path=String::from("./db/");
        _ = fs::create_dir_all(&common_path.clone());
        
        let mut tmp_dir=common_path.clone();
        tmp_dir.push_str("tmp/");
        _ = fs::create_dir_all(&tmp_dir.clone());
        
        let paths = fs::read_dir(tmp_dir);
        if paths.is_ok(){
            let mut file_list=vec![];
            let paths=paths.unwrap();
            for path in paths {
                if path.is_ok(){
                    let f_name=path.unwrap().path().display().to_string();
                    file_list.push(f_name.clone());
                }else{
                    println!("ERR : {}", path.unwrap().path().display());
                }
            }
            file_list.sort();
            for item in file_list{
                let mut delete_file=true;
                let contents_result = fs::read_to_string(item.clone());
                if contents_result.is_ok(){
                    let contents=contents_result.unwrap();
                    let item_part:Vec<String>= contents.split("||").map(|s| s.to_string()).collect();
                    if item_part.len()==2{
                        let key_result=hex::decode(&item_part[0]);
                        let value_result=hex::decode(&item_part[1]);
                        if key_result.is_ok() && value_result.is_ok(){
                            let key_val=String::from_utf8(key_result.unwrap());
                            let value_val=String::from_utf8(value_result.unwrap());
                            if key_val.is_ok() && value_val.is_ok(){
                                Fragmented::set_to_sub_db(
                                    fragmented_db_count,
                                    &key_val.unwrap(),
                                    &value_val.unwrap(),
                                    common_path.clone(),
                                    item.clone()
                                );
                                delete_file=false;
                            }
                        }
                    }
                }
                if delete_file==true{
                    _ = fs::remove_file(&item.clone());
                }
            }
        }
        Fragmented{
            fragmented_db_count:fragmented_db_count,
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
            let tmp_fragmented_db_count=self.fragmented_db_count;
            thread::spawn(move ||{
                Fragmented::set_to_sub_db(tmp_fragmented_db_count,&key_tmp,&value_tmp,db_path_tmp,tmp_file_cloned);
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
        let path_tmp=Fragmented::get_key_db_name(self.fragmented_db_count,key,self.path.clone());
        let mut tmp_db_obj=KeyValueDb::new(&path_tmp);
        let result_val=tmp_db_obj.get_value(&key);
        tmp_db_obj.close();
        if result_val.is_some(){
            return String::from(result_val.unwrap());
        }
        return String::from("");
    }
    pub fn remove(&mut self,key:&str){
        self.delete(key);
    }
    pub fn delete(&mut self,key:&str){
        self.list.remove(key);
        let path_tmp=Fragmented::get_key_db_name(self.fragmented_db_count,key,self.path.clone());
        let mut tmp_db_obj=KeyValueDb::new(&path_tmp);
        tmp_db_obj.delete(&key);
        tmp_db_obj.close();
    }
    pub fn get_key_db_name(fragmented_db_count:u16,key:&str,path_val:String)->String{
        let hash_result=goxoy_hash::hash::Hash::calculate(goxoy_hash::hash::HashKind::SHA1, &key.clone());
        let converted_hex = u64::from_str_radix(&hash_result[..10], 16);
        let mut part_no=u64::MAX;
        if converted_hex.is_ok(){
            part_no=converted_hex.unwrap();
        }
        format!("{}data_{}",path_val, format!("{:01$}", (part_no % (fragmented_db_count as u64)), fragmented_db_count.to_string().len()))
    }
    pub fn set_to_sub_db(fragmented_db_count:u16,key:&str,value:&str,path_val:String,tmp_file:String){
        let path_tmp=Fragmented::get_key_db_name(fragmented_db_count, key,path_val);
        let mut tmp_db_obj=KeyValueDb::new(&path_tmp);
        tmp_db_obj.set_value(&key, &value);
        tmp_db_obj.close();
        _ = fs::remove_file(&tmp_file);
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    let mut kv_obj=Fragmented::new(100);
    kv_obj.set_value("key-degeri-1","value-degeri-1");
    kv_obj.set_value("key-degeri-2","value-degeri-2");
    kv_obj.set_value("key-degeri-3","value-degeri-3");
    assert_eq!("value-degeri-1".to_string(),kv_obj.get_value("key-degeri-1"))
}
