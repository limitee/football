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
    new window.Com({id:self.main_id, path:"man_terminal_list", pins:self}, function(err, data){
        cb(err, data);
    });
}

Com.prototype.to_edit_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_terminal_edit", pins:self, add:add});
}

Com.prototype.to_account_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_account_account", pins:self, add:add});
}

Com.prototype.to_charge_report_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_account_charge.report", pins:self, add:add});
}

Com.prototype.to_charge_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_account_charge", pins:self, add:add});
}

Com.prototype.to_reset_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_account_reset", pins:self, add:add});
}

Com.prototype.to_game_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_terminal_game", pins:self, add:add});
}