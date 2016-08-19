var Com = function() {
    var self = this;
    self.data = {};
};

Com.prototype.init = function(cb) {
    var self = this;
    self.main_id = self.com.get_id("main");
    cb(null, null)
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var bar_item_id = 'li[flag="' + self.com.get_id("bar_item") + '"]';;
    var el = [{id:bar_item_id, on:"click", do:function(e){
        $(this).parent().find("li.active").removeClass("active");
        $(this).addClass("active");
        var url = $(this).attr("url");
        var status = parseInt($(this).attr("status"));
        var add = {
            cond: {
                status: status
            }
        }
        new window.Com({id:self.main_id, path:url, pins:self, add:add});
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    var add = {
        cond: {
        }
    }
    new window.Com({id:self.main_id, path:"man_pool_list", pins:self, add:add}, function(err, data){
        cb(err, data);
    });
}
