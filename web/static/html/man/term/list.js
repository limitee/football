var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 30,
        sort: [{game_id:-1}, {code:1}],
        cond: {
        },
        total: 0,
        set_list: [],
        game_rel: {}
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var add = self.com.cfg.add;
    if(add.cond.status > 0) {
        self.data.cond.status = add.cond.status;
        if(self.data.cond.status == 100) {
            self.data.sort = [{game_id:-1}, {code:-1}];
        }
    }
    self.pb_id = self.com.get_id("pagebar");

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game_rel[id].name;
    });

    juicer.unregister('get_term_status_des');
    juicer.register('get_term_status_des', function(id){
        return self.data.term_status[id].desc;
    });

    self.get_page_data(1, cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var item_id = 'a[flag="' + self.com.get_id("view_st") + '"]';
    var endtime_id = 'a[flag="' + self.com.get_id("endtime") + '"]';
    var gl_id = 'a[flag="' + self.com.get_id("gl") + '"]';
    var gl_cancel_id = 'a[flag="' + self.com.get_id("gl_cancel") + '"]';
    var search_id = self.com.get_jid("search");
    var el = [
        {id:item_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            //self.com.pins.to_detail_page(t_id);
        }},
        {id:endtime_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_edit_endtime_page(t_id);
        }},
        {id:gl_id, on:"click", do:function(e){
            var term_id = parseInt($(this).attr("t_id"));
            var term_code = parseInt($(this).attr("t_code"));
            var game_id = parseInt($(this).attr("t_game_id"));
            if(game_id == 202 || game_id == 201 || game_id == 301) {
                self.com.pins.to_draw_page(term_id, term_code, game_id);
            } else {
                self.com.pins.to_gl_page(term_id, term_code, game_id);
            }
        }},
        {id:gl_cancel_id, on:"click", do:function(e){
            var term_id = parseInt($(this).attr("t_id"));
            var term_code = parseInt($(this).attr("t_code"));
            var game_id = parseInt($(this).attr("t_game_id"));
            if(confirm("确定要取消" + term_code + "比赛吗?")) {
                self.cancel_term(term_id);
            }
        }},
        {id:search_id, on:"click", do:function(e){
            self.data.cur_page = 1;
            var game_id = parseInt(self.dom_game_id.val());
            if(game_id > 0) {
                self.data.cond.game_id = game_id;
            } else {
                delete self.data.cond.game_id;
            }
            self.to_page(1);
        }}
    ];
    cb(null, el);
}

Com.prototype.cancel_term = function(term_id) {
    var self = this;
    var body = {
        cond: {
            id: term_id,
            $or: [
                {status: 50},
                {status: 55}
            ]
        },
        doc: {
            $set: {
                draw_number:"*",
                status: 60
            }
        }
    };
    CurSite.postDigest({cmd:"AGT03"}, body, function(err, back_body)
    {
        if(self.data.cond.status) {
            self.com.pins.to_list_page_by_status(self.data.cond.status);
        } else {
            self.com.pins.to_list_page_by_status(-1);
        }
    });
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    var add = {
        skip: self.data.skip,
        limit: self.data.limit,
        total: self.data.total
    }

    self.dom_game_id = self.com.get("game_id");
    self.dom_modal = self.com.get("myModal");
    self.dom_modal_body = self.com.get("myModalBody");

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
    CurSite.postDigest({cmd:"AGT02"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
            self.data.game_list = back_body.game_list;
            self.data.term_status = back_body.term_status;

            for(var key in self.data.game_list) {
                var set = self.data.game_list[key];
                self.data.game_rel[set.id] = set;
            }
        }
        cb(null, null);
    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}
