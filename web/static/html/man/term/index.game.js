var Com = function() {
    var self = this;
    self.data = {};
};

Com.prototype.init = function(cb) {
    var self = this;
    self.main_id = self.com.get_id("main");
    CurSite.postDigest({cmd:"AG01"}, {}, function(err, back_body)
    {
        cb(null, null)
    });
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';;
    var el = [{id:bar_item_id, on:"click", do:function(e){
        $(this).parent().find("li.active").removeClass("active");
        $(this).addClass("active");
        var url = $(this).attr("url");
        new window.Com({id:self.main_id, path:url, pins:self});
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    new window.Com({id:self.main_id, path:"man_term_list.game", pins:self}, function(err, data){
        cb(err, data);
    });
}