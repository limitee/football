var Com = function() {
    var self = this;
    self.data = {};
};

Com.prototype.init = function(cb) {
    var self = this;
    self.main_id = self.com.get_id("main");
    CurSite.postDigest({cmd:"AG01"}, {}, function(err, back_body)
    {
        cb(null, null)
    });
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
    self.to_list_page(cb);
}

Com.prototype.to_list_page = function(cb) {
    var self = this;
    var add = {
        cond: {
            status: -1
        }
    };
    new window.Com({id:self.main_id, path:"man_term_list", pins:self, add:add}, function(err, data){
        if(cb) {
            cb(err, data);
        }
    });
}

Com.prototype.to_list_page_by_status = function(status, cb) {
    var self = this;
    var add = {
        cond: {
            status: status
        }
    };
    new window.Com({id:self.main_id, path:"man_term_list", pins:self, add:add}, function(err, data){
        if(cb) {
            cb(err, data);
        }
    });
}

Com.prototype.to_edit_endtime_page = function(id) {
    var self = this;
    var add = {
        id:id
    }
    new window.Com({id:self.main_id, path:"man_term_edit.endtime", pins:self, add:add});
}

Com.prototype.to_gl_page = function(term_id, term_code, game_id) {
    var self = this;
    var add = {
        term_id: term_id,
        term_code: term_code,
        game_id: game_id
    }
    new window.Com({id:self.main_id, path:"man_term_gl", pins:self, add:add});
}

Com.prototype.to_draw_page = function(term_id, term_code, game_id) {
    var self = this;
    if(game_id == 200) {
        var url = "man_term_draw.dlt";
    } else if (game_id == 202) {
        var url = "man_term_draw.jcd";
    }
    else if (game_id == 201) {
        var url = "man_term_draw.jcc";
    } else if(game_id == 301) {
        var url = "man_term_draw.jcl";
    } else {
        return;
    }
    var add = {
        term_id: term_id,
        term_code: term_code,
        game_id: game_id
    }
    new window.Com({id:self.main_id, path:url, pins:self, add:add});
}
