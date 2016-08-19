var Com = function() {
    var self = this;
    self.data = {};
}

Com.prototype.init = function(cb) {
    var self = this;
    var add = self.com.cfg.add;
    self.cur = add.skip/add.limit + 1; //current page
    self.page_count = parseInt(add.total/add.limit);
    if(add.total%add.limit > 0) {
        self.page_count++;
    }
    var index_array = self.get_index_array();
    self.data.sets = index_array;
    self.data.cur = self.cur;
    self.data.pIndex = self.cur - 1;
    self.data.nIndex = self.cur + 1;
    self.data.page_count = self.page_count;
    cb(null, null);
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    var index_id = 'a';
    var el = [{
        id:index_id, on:"click", do:function(e){
            var p_index = parseInt($(this).attr("pIndex"));
            if(p_index > 0 && p_index != self.data.cur && p_index <= self.page_count) {
                self.com.pins.to_page(p_index);
            }
        }
    }];
    cb(null, el);
}

Com.prototype.get_index_array = function() {
    var self = this;
    var page_count = self.page_count;
    var array = [];
    if(page_count < 7) {
        for(var i = 0; i < page_count; i++) {
            array.push(i + 1);
        }
    } else {
        if(self.cur < 4 || page_count - self.cur < 3) {
           array.push(1);array.push(2);array.push(3);
           array.push(-1);
           array.push(page_count - 2);array.push(page_count - 1);array.push(page_count);
        } else {
           array.push(1);array.push(-1);
           array.push(self.cur - 1);
           array.push(self.cur);
           array.push(self.cur + 1);
           array.push(-1);array.push(page_count);
        }
    }
    return array;
}
