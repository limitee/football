var Com = function() {
    var self = this;
};

Com.prototype.init = function(cb) {
    var self = this;
    self.com_id = self.com.cfg.add.id;
    var body = {
        id: self.com_id
    };
    CurSite.postDigest({cmd:"AC03"}, body, function(err, back_body)
    {

        if(back_body) {
            self.data.set = back_body.customer;
        }
        cb(null, null);
    });
}