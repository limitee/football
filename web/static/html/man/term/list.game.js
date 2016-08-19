var Com = function() {
    var self = this;
    self.data = {
        skip: -1,
        limit: -1,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        set_list: []
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    self.get_page_data(1, cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    //var item_id = 'a[flag="' + self.com.get_id("view_st") + '"]';
    ///var account_id = 'a[flag="' + self.com.get_id("account") + '"]';
    //var charge_id = 'a[flag="' + self.com.get_id("charge") + '"]';
    var el = [];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;

    self.dom_modal = self.com.get("myModal");
    self.dom_modal_body = self.com.get("myModalBody");
}

Com.prototype.get_page_data = function(index, cb) {
    var self = this;
    self.data.skip = (index - 1)*self.data.limit;
    var body = {
        cond: JSON.stringify(self.data.cond),
        sort: JSON.stringify(self.data.sort),
        offset: self.data.skip,
        limit: self.data.limit
    };
    CurSite.postDigest({cmd:"AG01"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
        }
        cb(null, null);
    });
}