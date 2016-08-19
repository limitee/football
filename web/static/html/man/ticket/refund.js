var Com = function() {
    var self = this;
    self.data = {
        game_list: [],
        game_id: -1
    }
};

Com.prototype.init = function(cb) {
    var self = this;

    var body = {};
    CurSite.postDigest({cmd:"AG01"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.game_list = back_body.data;
        }
        cb(null, null);
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;

    var sbt_id = self.com.get_jid("sub");
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';
    var el = [
        {id:sbt_id, on:"click", do:function(e){
            var bt = $(this);
            if(self.data.game_id < 0) {
                alert("请选择一个游戏");
                return;
            }
            var term_code = parseInt(self.dom_term_code.val());
            bt.button("loading");
            if(confirm("确定要退款吗？")) {
                var body = {
                    game_id: self.data.game_id,
                    term_code: term_code
                }
                CurSite.postDigest({cmd:"ATI11"}, body, function(err, back_body)
                {
                    if(err) {
                        alert(err.des);
                    } else {
                        alert("操作成功");
                    }
                    bt.button("reset");
                });
            }
        }},
        {id:bar_item_id, on:"click", do:function(e){
            $(this).parent().find("li.active").removeClass("active");
            $(this).addClass("active");

            self.data.game_id = parseInt($(this).attr("t_id"));
        }}
    ];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    self.dom_term_code = self.com.get("term_code");
    cb(null, null);
}

