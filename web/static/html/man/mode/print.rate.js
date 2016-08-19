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
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_gap = self.com.get("gap");
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
            self.data.cur_gap = back_body.cur_gap;
        }
        cb(null, null)
    });
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.gap = parseInt(self.dom_gap.val());
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.gap <= 0) {
        alert("间隔不能小于0");
        return false;
    }
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    var body = data;
    CurSite.postDigest({cmd:"AT09"}, body, function(err, back_body)
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