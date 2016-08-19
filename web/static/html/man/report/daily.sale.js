var Com = function() {
    var self = this;
    self.data = {
        cond: {
        },
        total: 0,
        total_bonus: 0,
        total_bat: 0,
        game_rel: {},
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var date = CurSite.getDateStr(); 
    self.data.cond.start_date = date.substr(0, 10);

    //var d = new Date();
    //var end_date = CurSite.getDateStr(d.getTime() + 24*60*60*1000); 
    self.data.cond.end_date = date.substr(0, 10);

    juicer.unregister('get_game_name');
    juicer.register('get_game_name', function(id){
        return self.data.game_rel[id + ""].name;
    });


    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var search_id = self.com.get_jid("search");
    var el = [
        {id:search_id, on:"click", do:function(e){
            var date = self.dom_date.val();
            self.data.cond.start_date = date;
            var end_date = self.dom_end_date.val();
            self.data.cond.end_date = end_date;
            self.refresh();
        }}
    ];
    cb(null, el);
}

Com.prototype.refresh = function(cb) {
    var self = this;
    self.data.total = 0;
    self.data.total_bonus = 0;
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
        end_date: self.data.cond.end_date
    };
    CurSite.postDigest({cmd:"AR03"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.tdetail = back_body.tdetail;
            for(var i = 0; i < self.data.tdetail.length; i++) {
                var set = self.data.tdetail[i];
                self.data.total += set.balance;
                self.data.total_bonus += set.bonus;
                self.data.total_bat += set.bat;
            }

            var game_rel = {};
            for(var key in back_body.game_list) {
                var set = back_body.game_list[key];
                game_rel[set.id + ""] = set;
            }
			self.data.game_rel = game_rel;
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

