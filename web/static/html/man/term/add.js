var Com = function() {
    var self = this;
    self.data = {
        game_id: -1,
        play_type_list: [
            {id:"01", name:"胜平负"},
            {id:"02", name:"让球胜平负"},
            {id:"03", name:"总进球数"},
            {id:"04", name:"比分"},
            {id:"05", name:"半全场"}
        ],
        play_map: {
            "01":1,
            "02":1,
            "03":1,
            "04":1,
            "05":1,
        },
        dc_play_map: {
            "01":0,
            "02":0,
            "03":1,
            "04":1,
            "05":1,
        }
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    self.data.sale_time = CurSite.getDateStr();
    self.data.end_time = CurSite.getDateStr();
    var body = {
        cond: "{}",
        sort: "{}",
        offset: -1,
        limit: -1
    };
    CurSite.postDigest({cmd:"AG01"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.game_list = back_body.data;
            if(self.data.game_list.length > 0) {
                self.data.game_id = self.data.game_list[0].id;
            }
        }
        cb(null, null);
    });
}

Com.prototype.get_play_types = function() {
    var self = this;
    var play_types = "";
    for(var key in self.data.play_map) {
        if(self.data.play_map[key] == 1) {
            if(play_types.length > 0) {
                play_types += "," + key;
            } else {
                play_types += key;
            }
        }
    }
    return play_types;
}

Com.prototype.get_dc_play_types = function() {
    var self = this;
    var play_types = "";
    for(var key in self.data.dc_play_map) {
        if(self.data.dc_play_map[key] == 1) {
            if(play_types.length > 0) {
                play_types += "," + key;
            } else {
                play_types += key;
            }
        }
    }
    return play_types;
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var pt_item_id = 'li[flag="' + self.com.get_id("pt_item") + '"]';
    var dc_item_id = 'li[flag="' + self.com.get_id("dc_item") + '"]';
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var data = self.get_data();
            if(self.check(data)) {
                self.save(data, function(err, data){
                    bt.button("reset");
                });
            } else {
                bt.button("reset");
            }
        }},
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.game_id = parseInt($(this).attr("t_id"));
        }},
        {id:pt_item_id, on:"click", do:function(e){
            var id = $(this).attr("t_id");
            if(self.data.play_map[id] == 0) {
                self.data.play_map[id] = 1;
                $(this).addClass("active");
            } else {
                self.data.play_map[id] = 0;
                $(this).removeClass("active");
            }
        }},
        {id:dc_item_id, on:"click", do:function(e){
            var id = $(this).attr("t_id");
            if(self.data.dc_play_map[id] == 0) {
                self.data.dc_play_map[id] = 1;
                $(this).addClass("active");
            } else {
                self.data.dc_play_map[id] = 0;
                $(this).removeClass("active");
            }
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_sale_time = self.com.get("sale_time");
    self.dom_end_time = self.com.get("end_time");
    self.dom_code = self.com.get("code");
    self.dom_master = self.com.get("master");
    self.dom_guest = self.com.get("guest");
    self.dom_give = self.com.get("give");
    cb(null, null);
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.game_id = self.data.game_id;
    data.sale_time = self.dom_sale_time.val();
    data.end_time = self.dom_end_time.val();
    data.master = self.dom_master.val();
    data.guest = self.dom_guest.val();
    data.give = self.dom_give.val();
    data.give = parseInt(parseFloat(data.give)*10);
    data.code = self.dom_code.val();

    data.play_types = self.get_play_types();
    data.dc_play_types = self.get_dc_play_types();

    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.code.length == 0) {
        alert("期次号不能为空");
        return false;
    }
    if(data.sale_time.length == 0) {
        alert("开售时间不能为空");
        return false;
    }
    if(data.end_time.length == 0) {
        alert("停售时间不能为空");
        return false;
    }
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    data.code = parseInt(data.code);
    var body = {
        data:data
    }
    CurSite.postDigest({cmd:"AGT01"}, body, function(err, back_body)
    {
        alert("操作成功")
        if(cb) {
            cb(null, null);
        }
    });
}
