var Com = function() {
    var self = this;
    self.data = {
        hard_id: 1,
        soft_id: 1
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    var body = {
        cond: JSON.stringify({}),
        sort: JSON.stringify([]),
        offset: -1,
        limit: -1
    };
    CurSite.postDigest({cmd:"CR02"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.hard_type = back_body.hard_type;
            self.data.soft_type = back_body.soft_type;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var hard_item_id = 'li[flag="' + self.com.get_id("hard_item") + '"]';
    var soft_item_id = 'li[flag="' + self.com.get_id("hard_item") + '"]';
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
        {id:hard_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.hard_id = parseInt($(this).attr("t_id"));
        }},
        {id:soft_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.soft_id = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_username = self.com.get("username");
    self.dom_password = self.com.get("password");
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.password = self.dom_password.val();
    data.hard_type = self.data.hard_id;
    data.soft_type = self.data.soft_id;
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.username.length == 0) {
        alert("用户名不能为空");
        return false;
    }
    if(data.password.length == 0) {
        alert("密码不能为空");
        return false;
    }
    return true;
}

Com.prototype.save = function(data, cb) {
    var self = this;
    var body = {
        data:data
    }
    CurSite.postDigest({cmd:"CRT02"}, body, function(err, back_body)
    {
        alert("操作成功")
        if(cb) {
            cb(null, null);
        }
    });
}
