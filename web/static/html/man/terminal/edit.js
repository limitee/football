var Com = function() {
    var self = this;
    self.data = {
        set: {

        },
        ext: {

        }
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    async.waterfall([
        function(cb) {
            self.term_id = self.com.cfg.add.id;
            var body = {
                terminal_id: self.term_id
            };
            CurSite.postDigest({cmd:"AT03"}, body, function(err, back_body)
            {
                if(back_body) {
                    self.data.set = back_body.terminal;
                    self.data.group_id = self.data.set.group_id;
                    self.data.ext = back_body.ext;
                    self.data.terminal_mode = back_body.terminal_mode;
                }
                cb(null);
            });
        }, 
        function(cb) {
            var body = {
                cond: JSON.stringify({}),
                sort: JSON.stringify([]),
                offset: -1,
                limit: -1
            };
            CurSite.postDigest({cmd:"ACT02"}, body, function(err, back_body)
            {
                if(back_body.data) {
                    self.data.group_list = back_body.data;
                }
                cb(null, null);
            });
        }
    ], function(err, data) {
        cb(null, null); 
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var sbt_id = self.com.get_jid("sub");
    var mode_item_id = 'li[flag="' + self.com.get_id("mode_item") + '"]';
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var el = [
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.group_id = parseInt($(this).attr("t_id"));
        }},
        {id:mode_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");
            self.data.ext.mode = parseInt($(this).attr("t_id"));
        }},
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var print_gap = parseInt(self.dom_print_gap.val());
            var help_bonus_id = parseInt(self.dom_help_bonus_id.val());
            var guarantee_amount = parseInt(self.dom_guarantee_amount.val())*100;
            var body = {
                cond: {
                    id: self.term_id
                },
                doc: {
                    $set: {
                        print_gap: print_gap,
                        help_bonus_id: help_bonus_id,
                        guarantee_amount: guarantee_amount,
                        mode: self.data.ext.mode
                    }
                },
                group_id: self.data.group_id
            };
            CurSite.postDigest({cmd:"AT12"}, body, function(err, back_body)
		    {
		        if(err) {
		            alert(err);
		        } else {
		            alert("操作成功");
		        }
		        bt.button("reset");
		    });
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_print_gap = self.com.get("print_gap");
    self.dom_help_bonus_id = self.com.get("help_bonus_id");
    self.dom_guarantee_amount = self.com.get("guarantee_amount");
    cb(null, null)
}
