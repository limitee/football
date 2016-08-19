var Com = function() {
    var self = this;
    self.data = {
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var el = [
    ];
    cb(null, el);
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
    };
    CurSite.postDigest({cmd:"AR01"}, body, function(err, back_body)
    {
        if(back_body) {
            self.data.detail = back_body;
        }
        cb(null, null);
    });
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    cb(null, null);
}

