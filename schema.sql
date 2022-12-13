create table if not exists shortened_urls (
    id varchar(6) primary key,
    url varchar(256) not null
);