var Com = function() {
    var self = this;
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var sbt_id = self.com.get_jid("reg_bt");
    var el = [{
        id:sbt_id, on:"click", do:function(e){
            var data = self.get_data();
            CurSite.setCookie("userId", data.username, -1);
            CurSite.setCookie("userType", "terminal", -1);
            var key = CryptoJS.MD5(data.password).toString(CryptoJS.enc.Hex);
            CurSite.postDigest({cmd:"T01", key:key}, {}, function(err, data){
                if(err) {
                    alert(err.des);
                }
                else
                {
                    //CurSite.setCookie("userId", data.userId, -1);
                    //CurSite.setCookie("st", data.st, -1);
                    CurSite.setCookie("st", key, -1);
                    window.location = "./terminal_manager.html";
                }
            });
        }
    }];
    cb(null, el);
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.password = self.dom_password.val();
    return data;
}

Com.prototype.page_loaded = function(e) {
    var self = this;
    self.dom_username = self.com.get("username");
    self.dom_password = self.com.get("password");
}