var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 20,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        cur_page: 1,
        set_list: [],
        group_map: {},
        select_group_map: {},
        province_map: {}
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    self.pb_id = self.com.get_id("pagebar");

    self.body_id = self.com.get_id("myModalBody");

    juicer.unregister('get_group_name');
    juicer.register('get_group_name', function(id){
        if(id < 0) {
            return "无";
        } else {
            return self.data.group_map[id].nickname;
        }
    });

    async.waterfall([
        function(cb) {  //get center info
            var body = {
                cond: JSON.stringify({}),
                sort: JSON.stringify([]),
                offset: -1,
                limit: -1
            };
            CurSite.postDigest({cmd:"ACT02"}, body, function(err, back_body)
            {
                if(back_body.data) {
                    for(var i = 0; i < back_body.data.length; i++) {
                        var set = back_body.data[i];
                        self.data.group_map[set.id] = set;
                    }
                }
                cb(null, null);
            });
        }
    ], function(err, data) {
        self.get_page_data(1, cb);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var province_item_id = 'li[flag="' + self.com.get_id("province_item") + '"]';
    var account_id = 'a[flag="' + self.com.get_id("account") + '"]';
    var charge_id = 'a[flag="' + self.com.get_id("charge") + '"]';
    var reset_id = 'a[flag="' + self.com.get_id("reset") + '"]';
    var item_id = 'a[flag="' + self.com.get_id("view_st") + '"]';
    var game_id = 'a[flag="' + self.com.get_id("game") + '"]';
    var to_man = 'a[flag="' + self.com.get_id("to_man") + '"]';
    var search_id = self.com.get_jid("search");
    var el = [
        {id:bar_item_id, on:"click", do:function(e){
            var group_id = $(this).attr("t_id");
            if(!self.data.select_group_map[group_id]) {
                $(this).addClass("active");
                self.data.select_group_map[group_id] = 1;
            } else {
                $(this).removeClass("active");
                self.data.select_group_map[group_id] = 0;
            }
        }},
        {id:province_item_id, on:"click", do:function(e){
            var group_id = $(this).attr("t_id");
            if(!self.data.province_map[group_id]) {
                $(this).addClass("active");
                self.data.province_map[group_id] = 1;
            } else {
                $(this).removeClass("active");
                self.data.province_map[group_id] = 0;
            }
        }},
        {id:search_id, on:"click", do:function(e){
            self.data.cur_page = 1;
             
            var id_text = self.dom_id.val();
            if(id_text.length > 0) {
                var id = parseInt(id_text);
                self.data.cond.id = id;
            } else {
                delete self.data.cond.id;
            }
            var username = self.dom_username.val();
            if(username.length > 0) {
                self.data.cond.username = username;
            } else {
                delete self.data.cond.username;
            }

            var group_array = [];
            for(var key in self.data.select_group_map) {
                if(self.data.select_group_map[key] == 1) {
                    group_array.push(key);
                }
            }
            if(group_array.length > 0) {
                self.data.cond.group_id = {"$in":group_array}
            } else {
                delete self.data.cond.group_id;
            }

            var province_array = [];
            for(var key in self.data.province_map) {
                if(self.data.province_map[key] == 1) {
                    province_array.push(key);
                }
            }
            if(province_array.length > 0) {
                self.data.cond.province = {"$in":province_array}
            } else {
                delete self.data.cond.province;
            }

            self.refresh();
        }},
        {id:item_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.dom_modal.modal('show');
            var add = {
                id:t_id
            }
            new window.Com({id:self.body_id, path:"man_terminal_edit", pins:self, add:add});
            /*
            var body = {terminal_id: t_id}
            CurSite.postDigest({cmd:"AT03"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err)
                    return;
                }
                var html = back_body.terminal.password;
                html += "<br/>";
                html += CryptoJS.MD5(back_body.terminal.password);
                self.dom_modal_body.html(html);
            });
            */
        }},
        {id:account_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_account_page(t_id);
        }},
        {id:charge_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_charge_page(t_id);
        }},
        {id:reset_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_reset_page(t_id);
        }},
        {id:game_id, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            self.com.pins.to_game_page(t_id);
        }},
        {id:to_man, on:"click", do:function(e){
            var t_id = parseInt($(this).attr("t_id"));
            var t_type = parseInt($(this).attr("t_type"));
            var body = {
                id:t_id,
                type:t_type
            };
            CurSite.postDigest({cmd:"AT10"}, body, function(err, back_body)
            {
                self.to_page(self.data.cur_page);
            });
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    self.dom_id = self.com.get("id");
    self.dom_username = self.com.get("username");

    var add = {
        skip: self.data.skip,
        limit: self.data.limit,
        total: self.data.total
    }

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
    CurSite.postDigest({cmd:"AT02"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;

            var map = {};
            for(var i = 0; i < back_body.ext.length; i++) {
                var set = back_body.ext[i];
                set.mode_des = back_body.terminal_mode[set.mode].desc;
                map[set.id] = set;
            }
            var a_map = {};
            for(var i = 0; i < back_body.accounts.length; i++) {
                var set = back_body.accounts[i];
                a_map[set.id] = set;
            }
            for(var i = 0; i < self.data.set_list.length; i++) {
                var set = self.data.set_list[i];
                set.ext = map[set.id];
                set.account = a_map[set.id];
            }
            self.data.province_type = back_body.province_type;
            for(var i = 0; i < self.data.set_list.length; i++) {
                var set = self.data.set_list[i];
                if(self.data.province_type[set.province]) {
                    set.province_des = self.data.province_type[set.province].desc;
                } else {
                    set.province_des = "未设置";
                }
            }
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
        self.com.refresh()
        if(cb) {
		    cb(null, null);
        }
    })
}
