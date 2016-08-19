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
        new window.Com({id:self.main_id, path:url, pins:self});
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    new window.Com({id:self.main_id, path:"man_doc_list", pins:self}, function(err, data){
        cb(err, data);
    });
}

Com.prototype.to_edit_page = function(id) {
    var self = this;
    var add = {
        doc_id:id
    }
    new window.Com({id:self.main_id, path:"man_doc_add", pins:self, add:add});
}