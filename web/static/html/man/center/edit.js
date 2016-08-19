var Com = function() {
    var self = this;
    self.data = {
        set: {

        }
    }
};

Com.prototype.init = function(cb) {
    var self = this;
    self.center_id = self.com.cfg.add.id;
    var body = {
        center_id: self.center_id
    };
    CurSite.postDigest({cmd:"ACT04"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.set = back_body.center;
            self.data.province_type = back_body.province_type;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var sbt_id = self.com.get_jid("sub");
    var province_item_id = 'li[flag="' + self.com.get_id("province_item") + '"]';
    var el = [
        {id:province_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.set.province = parseInt($(this).attr("t_id"));
        }},
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            bt.button("loading");
            var nickname = self.dom_nickname.val();
            var body = {
                center_id: self.center_id,
                province: self.data.set.province,
                nickname: nickname
            };
            CurSite.postDigest({cmd:"ACT05"}, body, function(err, back_body)
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
    self.dom_nickname = self.com.get("nickname");
    cb(null, null)
}
