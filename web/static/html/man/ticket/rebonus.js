var Com = function() {
    var self = this;
    self.data = {
        game_rel: {}
    }
};

Com.prototype.init = function(cb) {
    var self = this;

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game.name;
    });

    juicer.unregister('get_play_name');
    juicer.register('get_play_name', function(id){
        return self.data.play_type.name;
    });

    juicer.unregister('get_bet_name');
    juicer.register('get_bet_name', function(id){
        return self.data.bet_type.name;
    });

    juicer.unregister('get_ticket_status_des');
    juicer.register('get_ticket_status_des', function(id){
        return self.data.ticket_status[id].desc;
    });

    self.t_id = self.com.cfg.add.id;
    self.data.cur_time = self.com.cfg.add.cur_time;
    self.get_page_data(cb);
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
        id: self.t_id,
        cur_time: self.data.cur_time
    };
    CurSite.postDigest({cmd:"ATI02"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set = back_body.ticket;
            self.data.game_list = back_body.game_list;
            for(var key in self.data.game_list) {
                var set = self.data.game_list[key];
                if(set.id == self.data.set.game_id) {
                    self.data.game = set;
                    break;
                }
            }
            self.data.play_type = self.data.game.map[self.data.set.play_type];
            self.data.bet_type = self.data.play_type.map[self.data.set.bet_type];
            self.data.ticket_status = back_body.ticket_status;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var search_id = self.com.get_jid("search");
    var bonus_id = self.com.get_jid("bonus");
    var bonus_big_id = self.com.get_jid("bonus_big");
    var el = [
        {id:search_id, on:"click", do:function(e){
            var terminal_id = parseInt(self.dom_terminal_id.val());
            self.rebonus(terminal_id);
        }},
        {id:bonus_id, on:"click", do:function(e){
            var body = {
                id: self.t_id
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
        {id:bonus_big_id, on:"click", do:function(e){
            if(confirm("确定是大奖票吗？")) {
                var body = {
                    id: self.t_id
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
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    self.dom_terminal_id = self.com.get("terminal_id");
    cb(null, null);
}

Com.prototype.refresh = function() {
    var self = this;
    self.get_page_data(function(err, data){
        self.com.refresh();
    });
}

/**
 * 重新兑奖
 * @param id
 * @param cb
 */
Com.prototype.rebonus = function(terminal_id) {
    var self = this;
    var body = {
        id:self.t_id,
        terminal_id: terminal_id
    };
    CurSite.postDigest({cmd:"ATI05"}, body, function(err, back_body)
    {
        if(err) {
            alert(err.des);
        } else {
            alert("操作成功");
        }
    });
}
