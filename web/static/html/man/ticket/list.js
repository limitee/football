var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        set_list: [],
        game_rel: {},
        cur_page: 1,
        time_list:[
            {id:-1, des:"当前"}
        ],
        cur_time: -1
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var add = self.com.cfg.add;
    var status = add.cond.status;
    if(status > 0) {
        self.data.cond.status = status;
        if(status == 10 || status == 15 || status == 20 || status == 30
            || status == 50 || status == 52 ||
            status == 55 || status == 58 || status == 60 || status == 65) {
            self.data.sort = [{end_time:1}];
        }
    }
         
	self.pb_id = self.com.get_id("pagebar");
    self.detail_id = self.com.get_id("modal_body");

    self.get_page_data(1, cb);
}

Com.prototype.reset_juicer = function() {
    var self = this;

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game_rel[id + ""].name;
    });

    juicer.unregister('get_play_name');
    juicer.register('get_play_name', function(set){
        return self.data.game_rel[set.game_id + ""].map[set.play_type + ""].name;
    });

    juicer.unregister('get_bet_name');
    juicer.register('get_bet_name', function(set){
        return self.data.game_rel[set.game_id + ""].map[set.play_type + ""].map[set.bet_type + ""].name;
    });

    juicer.unregister('get_ticket_status_des');
    juicer.register('get_ticket_status_des', function(id){
        return self.data.ticket_status[id].desc;
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var item_id = 'a[flag="' + self.com.get_id("detail") + '"]';
    var print_id = 'a[flag="' + self.com.get_id("reprint") + '"]';
    var print_err_id = 'a[flag="' + self.com.get_id("print_err") + '"]';
    var refund_id = 'a[flag="' + self.com.get_id("refund") + '"]';
    var bonus_id = 'a[flag="' + self.com.get_id("bonus") + '"]';
    var bonus_big_id = 'a[flag="' + self.com.get_id("bonus_big") + '"]';
    var bonus_success_id = 'a[flag="' + self.com.get_id("bonus_success") + '"]';
    var stub_id = 'a[flag="' + self.com.get_id("stub") + '"]';
    var search_id = self.com.get_jid("search");
    var reprintall_id = self.com.get_jid("reprintall");
    var rebonusall_id = self.com.get_jid("rebonusall");
    var refund_match_id = self.com.get_jid("refund_match");
    var time_item_id = 'li[flag="' + self.com.get_id("time_item") + '"]';
    var el = [
        {id:item_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.dom_modal.modal("show");

            var add = {
                id:t_id,
                cur_time: self.data.cur_time
            }
            new window.Com({id:self.detail_id, path:"man_ticket_detail", pins:self, add:add});
            //self.com.pins.to_edit_page(t_id);
        }},
        {id:refund_match_id, on:"click", do:function(e){
            self.dom_modal.modal("show");
            new window.Com({id:self.detail_id, path:"man_ticket_refund", pins:self});
        }},
        {id:print_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            if(confirm("确定要重新出票吗？")) {
                var t_id = parseInt($(this).attr("t_id"));
                var cur_time = self.data.cur_time;
                self.com.pins.to_reprint_page(t_id, cur_time);
            }
        }},
        {id:bonus_big_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            if(confirm("确定是大奖票吗？")) {
                var t_id = parseInt($(this).attr("t_id"));
                var body = {
                    id: t_id
                }
                CurSite.postDigest({cmd:"ATI08"}, body, function(err, back_body)
                {
                    if(err) {
                        alert(err.des);
                    } else {
                        alert("操作成功");
                    }
                    self.refresh();
                });
            }
        }},
        {id:refund_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            if(confirm("确定要退款吗？")) {
                self.refund(t_id, function(err, data){
                    self.to_page(1)
                })
            }
        }},
        {id:print_err_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            if(confirm("确定要设置成出票错误吗？")) {
                self.print_err(t_id, function(err, data){
                    self.to_page(1)
                })
            }
        }},
        {id:bonus_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_rebonus_page(t_id, self.data.cur_time);
        }},
        {id:bonus_success_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            var body = {
                id: t_id
            }
            CurSite.postDigest({cmd:"ATI07"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                }
                self.refresh();
            });
        }},
        {id:stub_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.dom_modal.modal("show");

            var add = {
                id:t_id
            }
            new window.Com({id:self.detail_id, path:"man_ticket_stub", pins:self, add:add});
        }},
        {id:search_id, on:"click", do:function(e){
            self.data.cur_page = 1;
            var game_id = parseInt(self.dom_game_id.val());
            if(game_id > 0) {
                self.data.cond.game_id = game_id;
            } else {
                delete self.data.cond.game_id;
            }
            var id_text = self.dom_id.val();
            if(id_text) {
                self.data.cond.id = parseInt(id_text);
            } else {
                delete self.data.cond.id;
            }
            var out_id_text = self.dom_out_id.val();
            if(out_id_text) {
                self.data.cond.out_id = out_id_text;
            } else {
                delete self.data.cond.out_id;
            }
            var terminal_id_text = self.dom_terminal_id.val();
            if(terminal_id_text) {
                self.data.cond.terminal_id = terminal_id_text;
            } else {
                delete self.data.cond.terminal_id;
            }
            self.refresh();
        }},
        {id:reprintall_id, on:"click", do:function(e){
            var body = {
            };
            CurSite.postDigest({cmd:"ATI09"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                }
                self.refresh();
            });
        }},
        {id:rebonusall_id, on:"click", do:function(e){
            var body = {
            };
            CurSite.postDigest({cmd:"ATI10"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                }
                self.refresh();
            });
        }},
        {id:time_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.cur_time = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;

    self.dom_modal = self.com.get("modal");
    self.dom_game_id = self.com.get("game_id");
    self.dom_id = self.com.get("id");
    self.dom_out_id = self.com.get("out_id");
    self.dom_terminal_id = self.com.get("terminal_id");

    var add = {
        skip: self.data.skip,
        limit: self.data.limit,
        total: self.data.total
    }

    new window.Com({id:self.pb_id, path:"sys_pagebar", pins:self, add:add}, function(err, data){
        cb(null, null)
    });
}

Com.prototype.get_page_data = function(index, cb) {
    var self = this;
    self.data.skip = (index - 1)*self.data.limit;
    var body = {
        cond: self.data.cond,
        op: {
            sort: self.data.sort,
            offset: self.data.skip,
            limit: self.data.limit
        },
        cur_time: self.data.cur_time
    };
    CurSite.postDigest({cmd:"ATI01"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;

	        var time_list = [{id:-1, des:"当前"}];
            var year = back_body.cur_year;
            var month = back_body.cur_month;
            for(var i = 0; i < 5; i++) {
                var time_info = self.get_cur_time(year, month); 
                if(time_info) {
                    time_list.push(time_info);
                }
                month -= 1;
                if(month == 0) {
                    month = 12;
                    year -= 1;
                }
            }
            self.data.time_list = time_list;
		
			var game_rel = {};
            for(var key in back_body.game_list) {
                var set = back_body.game_list[key];
                game_rel[set.id + ""] = set;
            }
			self.data.game_rel = game_rel;
            self.data.ticket_status = back_body.ticket_status;

			self.reset_juicer();
        }
        cb(null, null);
    });
}

Com.prototype.get_cur_time = function(year, month) {
    if(year <= 2016 && month < 5) {
        return null;
    }
    var time_info = {id: year*100 + month, des:(year*100 + month) + ""};
    return time_info;
}

Com.prototype.refresh = function() {
    var self = this;
    self.to_page(self.data.cur_page, function(err, data){

    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.data.cur_page = index;
    self.get_page_data(index, function(err, data){
        self.com.refresh();
		cb(null, null);
    })
}

/**
 * 重新出票
 * @param id
 * @param cb
 */
Com.prototype.reprint = function(id, cb) {
    var self = this;
    var body = {
        id:id
    };
    CurSite.postDigest({cmd:"ATI03"}, body, function(err, back_body)
    {
        if(err) {
            alert(err.des)
        } else {
            alert("操作成功")
        }
        cb(null, null);
    });
}

/**
 * 重新出票
 * @param id
 * @param cb
 */
Com.prototype.refund = function(id, cb) {
    var self = this;
    var body = {
        id:id
    };
    CurSite.postDigest({cmd:"ATI04"}, body, function(err, back_body)
    {
        if(err) {
            alert(err.des)
        } else {
            alert("操作成功")
        }
        cb(null, null);
    });
}

/**
 * 重新出票
 * @param id
 * @param cb
 */
Com.prototype.print_err = function(id, cb) {
    var self = this;
    var body = {
        id:id
    };
    CurSite.postDigest({cmd:"ATI06"}, body, function(err, back_body)
    {
        if(err) {
            alert(err.des)
        } else {
            alert("操作成功")
        }
        cb(null, null);
    });
}
