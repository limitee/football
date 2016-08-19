var Com = function() {
    var self = this;
    self.data = {
        play_type_list: [
            {id:"01", name:"胜平负"},
            {id:"02", name:"让球胜平负"},
            {id:"03", name:"总进球数"},
            {id:"04", name:"比分"},
            {id:"05", name:"半全场"}
        ],
        jcl_play_type_list: [
            {id:"01", name:"胜负"},
            {id:"02", name:"让分胜负"},
            {id:"03", name:"胜分差"},
            {id:"04", name:"大小分"},
        ],
        play_map: {
            "01":0,
            "02":0,
            "03":0,
            "04":0,
            "05":0,
        },
        dc_play_map: {
            "01":0,
            "02":0,
            "03":0,
            "04":0,
            "05":0,
        }
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    self.term_id = self.com.cfg.add.id;
    var body = {
        id: self.term_id
    };
    CurSite.postDigest({cmd:"AGT04"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set = back_body.term;
            self.data.status_list = back_body.term_status;
            var play_types = self.data.set.play_types;
            if(play_types.length > 0) {
                var array = play_types.split(",");
                for(var key in array) {
                    var play_type = array[key];
                    self.data.play_map[play_type] = 1;
                }
            }

            var dc_play_types = self.data.set.dc_play_types;
            if(dc_play_types.length > 0) {
                var array = dc_play_types.split(",");
                for(var key in array) {
                    var play_type = array[key];
                    self.data.dc_play_map[play_type] = 1;
                }
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
    var status_item_id = 'li[flag="' + self.com.get_id("status_item") + '"]';
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var dc_item_id = 'li[flag="' + self.com.get_id("dc_item") + '"]';
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var sale_time = self.dom_sale_time.val();

            var str = sale_time.replace(/-/g,"/");
            var date = new Date(str);
            sale_time = date.getTime()/1000;

            var endtime = self.dom_end_time.val();
            var str = endtime.replace(/-/g,"/");
            var date = new Date(str);
            endtime = date.getTime()/1000;
            var play_types = self.get_play_types();
            var dc_play_types = self.get_dc_play_types();
            var give = parseInt(self.dom_give.val())*10;
            var code = parseInt(self.dom_code.val());
            var body = {
                cond: {
                    id: self.term_id
                },
                doc: {
                    $set: {
                        code: code,
                        sale_time: sale_time,
                        end_time: endtime,
                        play_types: play_types,
                        dc_play_types: dc_play_types,
                        status: self.data.set.status,
                        give:give
                    }
                }
            };
            CurSite.postDigest({cmd:"AGT03"}, body, function(err, back_body)
		    {
		        if(err) {
		            alert(err);
		        } else {
		            alert("操作成功");
		        }
		        bt.button("reset");
		    });
        }},
        {id:bar_item_id, on:"click", do:function(e){
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
        }},
        {id:status_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.set.status = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_end_time = self.com.get("end_time");
    self.dom_sale_time = self.com.get("sale_time");
    self.dom_give = self.com.get("give");
    self.dom_code = self.com.get("code");
    cb(null, null)
}
