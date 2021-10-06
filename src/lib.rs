use reqwest::blocking::multipart;

const APIURL:&str = "https://catbox.moe/user/api.php";

pub fn uploadfile(userhash: String, filepath: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "fileupload")
            .text("userhash", userhash)
            .file("fileToUpload", filepath).unwrap();
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("File upload failed")}
    }
}

pub fn uploadurl(userhash: String, url: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "urlupload")
            .text("userhash", userhash)
            .text("url", url);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Url upload failed")}
    }
}

pub fn deletefiles(userhash: String, files: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "deletefiles")
            .text("userhash", userhash)
            .text("files", files);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("File delete failed")}
    }
}

pub fn createalbum(userhash: String, title: String,desc: String, files: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "createalbum")
            .text("userhash", userhash)
            .text("title", title)
            .text("desc", desc)
            .text("files", files);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Album creation failed")}
    }
}

pub fn editalbum(userhash: String, short: String,title: String,desc: String, files: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "editalbum")
            .text("userhash", userhash)
            .text("short", short)
            .text("title", title)
            .text("desc", desc)
            .text("files", files);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Album edit failed")}
    }
}

pub fn addtoalbum(userhash: String, short: String,files: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "addtoalbum")
            .text("userhash", userhash)
            .text("short", short)
            .text("files", files);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Adding to album failed")}
    }
}

pub fn removefromalbum(userhash: String, short: String,files: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "removefromalbum")
            .text("userhash", userhash)
            .text("short", short)
            .text("files", files);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Removing from album failed")}
    }
}

pub fn deletealbum(userhash: String, short: String) -> String{
    let form = multipart::Form::new()
            .text("reqtype", "deletealbum")
            .text("userhash", userhash)
            .text("short", short);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(APIURL)
            .multipart(form).send().unwrap();
    match resp.text(){
        Ok(s) => {s}
        Err(_) => {String::from("Deleting album failed")}
    }
}