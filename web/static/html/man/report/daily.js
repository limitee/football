var Com = function() {
    var self = this;
    self.data = {
        cond: {
        },
        terminal_total: 0,
        game_list: [],
        select_game_map: {},
        time_list:[
            {id:0, name:"00:00:00-00:00:00"},
            {id:1, name:"06:00:00-06:00:00"}
        ],
        time_type: 0
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var date = CurSite.getDateStr(); 
    self.data.cond.start_date = date.substr(0, 10);

    //var d = new Date();
    //var end_date = CurSite.getDateStr(d.getTime() + 24*60*60*1000); 
    self.data.cond.end_date = date.substr(0, 10);

    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var time_item_id = 'li[flag="' + self.com.get_id("time_item") + '"]';
    var search_id = self.com.get_jid("search");
    var el = [
        {id:search_id, on:"click", do:function(e){
            var date = self.dom_date.val();
            self.data.cond.start_date = date;
            var end_date = self.dom_end_date.val();
            self.data.cond.end_date = end_date;

            var game_ids = [];
            for(var key in self.data.select_game_map) {
                if(self.data.select_game_map[key] == 1) {
                    game_ids.push(key);
                }
            }
            self.data.cond.game_ids = game_ids;

            self.refresh();
        }},
        {id:bar_item_id, on:"click", do:function(e){
            var group_id = $(this).attr("t_id");
            if(!self.data.select_game_map[group_id]) {
                $(this).addClass("active");
                self.data.select_game_map[group_id] = 1;
            } else {
                $(this).removeClass("active");
                self.data.select_game_map[group_id] = 0;
            }
        }},
        {id:time_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.time_type = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.refresh = function(cb) {
    var self = this;
    self.data.terminal_total = 0;
    self.get_page_data(function(err, data){
        self.com.refresh();
        if(cb) {
		    cb(null, null);
        }
    })
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
        start_date: self.data.cond.start_date,
        end_date: self.data.cond.end_date,
        game_ids: self.data.cond.game_ids,
        time_type: self.data.time_type
    };
    CurSite.postDigest({cmd:"AR02"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.tdetail = back_body.tdetail;
            for(var i = 0; i < self.data.tdetail.length; i++) {
                var set = self.data.tdetail[i];
                self.data.terminal_total += set.balance;
            }

            self.data.game_list = back_body.game_list;
        }
        cb(null, null);
    });
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    self.dom_date = self.com.get("start_date");
    self.dom_end_date = self.com.get("end_date");
    cb(null, null);
}

