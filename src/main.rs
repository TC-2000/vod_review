fn main() {
    let connection = sqlite::open(":memory:").unwrap();   

    let query = "
    CREATE TABLE videos (videoname TEXT, characterone TEXT, charactertwo TEXT, notes TEXT);
    INSERT INTO videos VALUES ('Testvideoname', 'testcharacterone', 'testcharactertwo', 'notes');
    INSERT INTO videos VALUES ('Testvideoname', 'testcharacterone', 'testcharactertwo', 'notes');
    "; 
    connection.execute(query).unwrap(); 
}
