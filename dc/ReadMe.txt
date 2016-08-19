添加企业测试用户
｀｀｀
insert into customer(username, password, nickname, type) values('com001', '123456', '123456', 300);
｀｀｀

添加出票中心
```
insert into customer(username, password, nickname, type) values('station001', '123456', '123456', 400);
```

添加管理员
```
insert into customer(username, password, nickname, type) values('admin', 'admin123', 'admin', 900);
```

20160401
```
create table moneylog (amount bigint default 0,create_time bigint default -1,customer_id bigint default -1,id bigserial PRIMARY KEY,mafter bigint default 0,mbefore bigint default 0,status integer default 0);
create table account (balance bigint default 0,id bigserial primary key,versoin bigint default 0);
```