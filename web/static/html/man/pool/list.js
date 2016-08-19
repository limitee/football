var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{end_time:1}, {amount:-1}],
        cond: {},
        total: 0,
        set_list: [],
        game_rel: {},
        cur_page: 1,
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var add = self.com.cfg.add;
    if(add.cond.status >= 0) {
        self.data.cond.status = add.cond.status;
    }
    
    if(self.data.cond.status == 1) {
        self.data.limit = 100;
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
        if(id == 0) {
            return "等待发送";
        } else if (id == 1) {
            return "已经发送";
        } else {
            return "未知";
        }
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var resend_id = 'a[flag="' + self.com.get_id("resend") + '"]';
    var search_id = self.com.get_jid("search");
    var el = [
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
            self.refresh();
        }},
        {id:resend_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            var version = parseInt($(this).attr("version"));
            if(confirm("确定要重新发送吗？")) {
                var body = {
                    id: t_id,
                    version
                }
                CurSite.postDigest({cmd:"ATI13"}, body, function(err, back_body)
                {
                    if(err) {
                        alert(err.des);
                    } else {
                        alert("操作成功");
                    }
                    self.refresh();
                });
            }
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;

    self.dom_modal = self.com.get("modal");
    self.dom_game_id = self.com.get("game_id");
    self.dom_id = self.com.get("id");

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
        cond: JSON.stringify(self.data.cond),
        sort: JSON.stringify(self.data.sort),
        offset: self.data.skip,
        limit: self.data.limit
    };
    CurSite.postDigest({cmd:"ATI12"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
			
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
