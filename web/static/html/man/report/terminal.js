var Com = function() {
    var self = this;
    self.data = {
        add: {
        }
    }
}

Com.prototype.init = function(cb) {
    var self = this;
    var date = CurSite.getDateStr(); 
    self.data.add.start_date = date.substr(0, 10);
    self.data.add.end_date = date.substr(0, 10);

    self.get_page_data(cb);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var search_id = self.com.get_jid("search");
    var el = [
        {id:search_id, on:"click", do:function(e){
            self.data.add.start_date = self.dom_start_date.val();
            self.data.add.end_date = self.dom_end_date.val();
            self.refresh();
        }}
    ];
    cb(null, el);
}

Com.prototype.refresh = function(cb) {
    var self = this;
    self.get_page_data(function(err, data){
        self.com.refresh();
        if(cb) {
		    cb(null, null);
        }
    });
}

Com.prototype.get_page_data = function(cb) {
    var self = this;
    var body = {
        add: self.data.add
    };
    CurSite.postDigest({cmd:"AR04"}, body, function(err, back_body)
    {
        console.log(back_body);
        if(back_body) {
            self.data.sets = back_body.sets;
        }
        cb(null, null);
    });
}

Com.prototype.page_loaded = function (cb) {
    var self = this;
    self.dom_start_date = self.com.get("start_date");
    self.dom_end_date = self.com.get("end_date");
    cb(null, null);
}

