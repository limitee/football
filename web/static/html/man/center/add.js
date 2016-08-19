var Com = function() {
    var self = this;
};

Com.prototype.init = function(cb) {
    var self = this;
    cb(null, null)
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var el = [{id:sbt_id, on:"click", do:function(e){
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
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_username = self.com.get("username");
    self.dom_nickname = self.com.get("nickname");
    self.dom_password = self.com.get("password");
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.nickname = self.dom_nickname.val();
    data.password = self.dom_password.val();
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.username.length == 0) {
        alert("用户名不能为空");
        return false;
    }
    if(data.nickname.length == 0) {
        alert("名称不能为空");
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
    CurSite.postDigest({cmd:"ACT01"}, body, function(err, back_body)
    {
        alert("操作成功")
        if(cb) {
            cb(null, null);
        }
    });
}
