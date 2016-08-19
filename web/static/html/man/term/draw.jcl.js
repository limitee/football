var Com = function() {
    var self = this;
    self.data = {
        num_list:["", ""]
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    self.data.term_id = self.com.cfg.add.term_id;
    self.data.term_code = self.com.cfg.add.term_code;
    self.data.game_id = self.com.cfg.add.game_id;

    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var next_id = self.com.get_jid("next");
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var draw_number = self.get_draw_number();

            var body = {
                cond: {
                    id: self.data.term_id
                },
                doc: {
                    $set: {
                        draw_number:draw_number
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
                self.refresh();
            });
        }},
        {id:next_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var body = {
                term_id: self.data.term_id,
                version: self.data.set.version
            };
            CurSite.postDigest({cmd:"AGT07"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                    self.com.pins.to_list_page_by_status(50);
                }
            });
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_red_1 = self.com.get("red_1");
    self.dom_blue_1 = self.com.get("blue_1");

    cb(null, null)
}

Com.prototype.get_draw_number = function() {
    var self = this;
    var red_1 = self.dom_red_1.val();
    var blue_1 = self.dom_blue_1.val();

    var number = red_1 + "," + blue_1;
    return number;
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    async.waterfall([
        function(cb) {
            var body = {
                term_id: self.data.term_id,
                term_code: self.data.term_code,
                game_id: self.data.game_id
            };
            CurSite.postDigest({cmd:"AGT05"}, body, function(err, back_body)
            {
                if(back_body) {
                    self.data.db_list = back_body.db_list;
                }
                cb(null);
            });
        },
        function(cb) {
            var body = {
                id: self.data.term_id
            };
            CurSite.postDigest({cmd:"AGT04"}, body, function(err, back_body)
            {
                if(back_body) {
                    self.data.set = back_body.term;
                    console.log(self.data.set);

                    if(self.data.set.draw_number.length > 0) {
                        var ar = self.data.set.draw_number.split(/,|\|/);
                        self.data.num_list[0] = parseInt(ar[0]);
                        self.data.num_list[1] = parseInt(ar[1]);
                    }
                }
                cb(null, null);
            });
        }
    ], function(err, data){
        cb(err, data)
    })
}

Com.prototype.refresh = function() {
    var self = this;
    self.get_page_data(function(err, data) {
        self.com.refresh()
    });
}
