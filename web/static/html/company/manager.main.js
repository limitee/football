var Com = function() {
    var self = this;
    self.data = {
        groups:[
            {name:"基础管理", items:[
                {name:"公司信息", url:"company_base_info"},
                {name:"模拟投注", url:"company_base_bet"}
            ]}
        ]
    };
}

Com.prototype.init = function(cb) {
    var self = this;
    self.main_id = self.com.get_id("main");
    cb(null, null)
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var url_item_id = 'a[flag="' + self.com.get_id("url_item") + '"]';
    var el = [{id:url_item_id, on:"click", do:function(e){
        var t_url = $(this).attr("t_url");
        new window.Com({id:self.main_id, path:t_url, pins:self});
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    new window.Com({id:self.main_id, path:"company_base_info", pins:self}, cb);
}
