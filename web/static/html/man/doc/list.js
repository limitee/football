var Com = function() {
    var self = this;
    self.data = {
        skip: 0,
        limit: 10,
        sort: [{id:-1}],
        cond: {},
        total: 0,
        set_list: []
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    self.pb_id = self.com.get_id("pagebar");

    self.get_page_data(1, cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var item_id = 'a[flag="' + self.com.get_id("edit") + '"]';
    var el = [{id:item_id, on:"click", do:function(e){
        var t_id = parseInt($(this).attr("t_id"));
        self.com.pins.to_edit_page(t_id);
    }}];
    cb(null, el);
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    var add = {
        skip: self.data.skip,
        limit: self.data.limit,
        total: self.data.total
    }
    new window.Com({id:self.pb_id, path:"sys_pagebar", pins:self, add:add}, function(err, data){
        cb(null, null)
    });
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
    CurSite.postDigest({cmd:"AD01"}, body, function(err, back_body)
    {
        if(back_body.data) {
            self.data.set_list = back_body.data;
            self.data.total = back_body.count;
        }
        cb(null, null);
    });
}

Com.prototype.to_page = function(index, cb) {
    var self = this;
    self.get_page_data(index, function(err, data){
        self.com.refresh()
    })
}


