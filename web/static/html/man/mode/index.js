var Com = function() {
    var self = this;
    self.data = {
        mode_list: []
    };
};

Com.prototype.init = function(cb) {
    var self = this;
    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var sbt_id = self.com.get_jid("sub");
    var jcc_term_id = self.com.get_jid("jcc_term");
    var jcl_term_id = self.com.get_jid("jcl_term");
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var resend_item_id = 'li[flag="' + self.com.get_id("resend_item") + '"]';
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var data = self.get_data();
            if(self.check(data)) {
                self.save(data, function(err, data){
                    self.refresh()
                    //bt.button("reset");
                });
            } else {
                bt.button("reset");
            }
        }},
        {id:jcc_term_id, on:"click", do:function(e){
            var body = {
                game_id: 201
            };
            CurSite.postDigest({cmd:"AT11"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                }
            });
        }},
        {id:jcl_term_id, on:"click", do:function(e){
            var body = {
                game_id: 301
            };
            CurSite.postDigest({cmd:"AT11"}, body, function(err, back_body)
            {
                if(err) {
                    alert(err.des);
                } else {
                    alert("操作成功");
                }
            });
        }},
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.cur_mode = parseInt($(this).attr("t_id"));
        }},
        {id:resend_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.resend = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    cb(null, null)
}

Com.prototype.refresh = function() {
    var self = this;
    self.get_page_data(function(err, data) {
        self.com.refresh()
    });
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
    };
    CurSite.postDigest({cmd:"AT07"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.mode_list = back_body.sys_mode;
            self.data.cur_mode = back_body.cur_mode;
            self.data.resend = back_body.resend;
        }
        cb(null, null)
    });
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.mode = self.data.cur_mode;
    data.resend = self.data.resend;
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    var body = data;
    CurSite.postDigest({cmd:"AT08"}, body, function(err, back_body)
    {
        //alert("操作成功")
        if(err) {
            alert(err.des);
        } else {
            alert("操作成功");
        }
        if(cb) {
            cb(null, null);
        }
    });
}
