# lot system

ssh root@123.56.177.229
ssh root@121.42.197.2

#web
```
scp ./target/debug/web root@123.57.64.210:/data/workspace/lot/web/target/debug/.
scp ./target/debug/web root@121.42.197.2:/data/workspace/lot/web/target/debug/.
```

```
nohup ./target/release/web name=lot > nohup.out 2>&1 &
nohup ./target/release/web name=lot target=admin > admin.out 2>&1 &
```

#server
```
scp ./target/debug/server root@123.57.64.210:/data/workspace/lot/server/target/debug/.
scp ./target/debug/server root@121.42.197.2:/data/workspace/lot/server/target/debug/.
```

```
nohup ./target/release/server name=lot_server > nohup.out 2>&1 &
```

#scheduler
```
scp ./target/debug/scheduler root@123.57.64.210:/data/workspace/lot/scheduler/target/debug/.
scp ./target/debug/scheduler root@121.42.197.2:/data/workspace/lot/scheduler/target/debug/.
```

```
nohup ./target/release/scheduler name=lot_scheduler > nohup.out 2>&1 &
```

#report
```
nohup ./target/release/report name=report > nohup.out 2>&1 &
```

#picker
```
nohup ./target/release/picker name=picker > nohup.out 2>&1 &
```

#protocol
```
nohup ./target/debug/protocol name=protocol > nohup.out 2>&1 &
```

```
/usr/local/pgsql/bin/initdb -D /usr/local/pgsql/data
```

```
/usr/local/pgsql/bin/createdb order_sys
```

#添加用户
```
CREATE ROLE postgres superuser;
```

#修改密码
```
ALTER ROLE postgres WITH PASSWORD 'bb3RrH8nrwUtN4eq';
```

#赋予登录权限
```
ALTER ROLE postgres WITH login;
```

导出term
```
/usr/local/pgsql/bin/pg_dump mcp -a -t term > ./mcp_term.sql
```

导入
```
/usr/local/pgsql/bin/psql mcp < ./mcp_term.sql
```

停止数据库
```
/usr/local/pgsql/bin/pg_ctl -D /usr/local/pgsql/data stop
```

创建索引
```
CREATE INDEX ticket_game_id_term_code_status ON ticket (game_id, term_code, status);

CREATE UNIQUE INDEX term_game_code ON term (game_id, code);
CREATE UNIQUE INDEX terminal_game_unique ON terminal_game (terminal_id, game_id);
CREATE UNIQUE INDEX game_level_unique ON game_level (game_id, term_code, lev);
CREATE UNIQUE INDEX ticket_customer_id_out_id ON ticket (customer_id, out_id);

CREATE UNIQUE INDEX charge_report_terminal_time ON charge_report (terminal_id, timestamp);

create index ticket_status on ticket (status) where status=10 or status=15 or status=20 or status=30 or status=50 or status=60 or status=65;

CREATE INDEX ticket_create_time ON ticket (create_time);
CREATE INDEX moneylog_create_time ON moneylog (create_time);
CREATE INDEX moneylog_customer_id ON moneylog (customer_id);
CREATE INDEX ticket_print_time ON ticket (print_time);
CREATE INDEX ticket_bonus_time ON ticket (bonus_time);
CREATE UNIQUE INDEX terminal_sale_terminal_date ON terminal_sale (terminal_id, sale_date);
```

```
update term set status=10, end_time=2460571376 where id=10;

//draw
update term set status=80 where id=60;

//bonus
update term set status=80 where id=8;

//draw_jcc
update term set status=60 where id=10;

//reset the ticket status to print success
update ticket set status=20, draw_code_list='';

#大乐透
```
{"tickets":[{"game_id":200,"play_type":10,"bet_type":10,"multiple":1,"number":"01,02,03,04,05|01,12","icount":1,"amount":200,"term_code":2016056}]}
```

{"tickets":[{"game_id":202,"play_type":5,"bet_type":1,"multiple":1,"number":"20160417001:00,11,33|1*1","icount":3,"amount":600}]}

//串关，胜平负
{"tickets":[{"game_id":201,"play_type":1,"bet_type":2,"multiple":1,"number":"20160417001:0,1,3;20160417002:0,1,3|2*1","icount":9,"amount":1800}]}

//混投
{"tickets":[{"game_id":201,"play_type":10,"bet_type":2,"multiple":1,"number":"20160417001:01:0,1,3;20160417002:03:1,2,3|2*1","icount":9,"amount":1800}]}

{"tickets":[{"game_id":201,"play_type":10,"bet_type":2,"out_id":"0001","multiple":1,"number":"20160417001:05:33;20160417002:01:1,3|2*1","icount":2,"amount":400}]}


alter table moneylog add column order_id varchar(80) default '';
alter table term add column draw_number varchar(200) default '';

alter table ticket add column bonus bigint default 0;
alter table ticket add column bonus_after_tax bigint default 0;
alter table ticket add column bonus_detail text default '';

alter table ticket add column term_code_list text default '';
alter table ticket add column print_number text default '';

alter table term add column master varchar(80) default '';
alter table term add column guest varchar(80) default '';
alter table term add column give integer default 0;

alter table ticket add column bonus_stub text default '';
alter table ticket add column draw_code_list text default '';

alter table terminal add column type int default 0;
alter table customer add column group_id bigint default -1;

alter table terminal add column soft_type int default -1;
alter table terminal add column hard_type int default -1;

alter table term add column play_types varchar(40) default '';
alter table term add column dc_play_types varchar(40) default '';

alter table terminal add column print_gap int default 6;
alter table terminal add column client_balance bigint default 0;
alter table terminal add column server_balance bigint default 0;

alter table account add column client_balance bigint default 1000000;
alter table ticket add column end_time bigint default -1;
alter table account alter column client_balance set default 0;
alter table ticket add column bonus_terminal_id bigint default -1;

alter table customer add column province int default -1;
alter table terminal_game add column scale int default 80;
alter table terminal add column mode int default 100;
alter table terminal add column help_bonus_id bigint default -1;
alter table terminal add column guarantee_amount int default 100000;
alter table ticket add column crypto text not null default '';
alter table ticket add column bonus_try_count int default 0;
alter table ticket add column bonus_time bigint default -1;
alter table terminal_sale add column bonus_err_count bigint default 0;
alter table terminal_sale add column bonus_err_amount bigint default 0;
```

# lot
lot system

#sql
##销售报表
```
select game_id,play_type,bet_type,count(*),sum(amount)/100 as amount,sum(bonus)/100 as bonus from ticket where customer_id=3 and (status=20 or status=30 or status=40 or status=60 or status=65 or status=70) group by game_id, play_type, bet_type order by game_id,play_type,bet_type;

select count(*),sum(amount)/100 as amount,sum(bonus)/100 as bonus from ticket where customer_id=3 and (status=20 or status=30 or status=40 or status=60 or status=65 or status=70);
```

update term set status=60 where game_id=201 and (code in (20160515006));

update term set status=60 where game_id=201 and (code > (20160518021) and code <= 20160519012);

update ticket set status=20 where status=40;

update ticket set status=20 where status = 40 or status=30 or status=60 or status=65 or status=70;


update ticket set status=70 where status=20 and id <= 2568;

update ticket set bonus=0,bonus_after_tax=0,bonus_detail='[]' where status=40;

update customer set province=1 where username like 'hb%';
update customer set province=2 where username like 'he%';
update customer set province=3 where username like 'sx%';
=======

# football
football analyse
