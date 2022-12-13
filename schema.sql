create table shortened_urls if not exists {
    id varchar(6) primary key,
    url varchar(256)
}