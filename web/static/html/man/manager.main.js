var Com = function() {
    var self = this;
    self.data = {
        groups:[
            {name:"基础管理", items:[
                {name:"接入文档", url:"man_doc_index"},
                {name:"系统模式", url:"man_mode_index"},
                {name:"出票速度", url:"man_mode_print.rate"}
            ]},
            {name:"用户管理", items:[
                {name:"终端机", url:"man_terminal_index"},
                {name:"出票中心", url:"man_center_index"},
                {name:"销售渠道", url:"man_company_index"}
            ]},
            {name:"游戏管理", items:[
                {name:"游戏管理", url:"man_term_index.game"},
                {name:"期次管理", url:"man_term_index"},
            ]},
            {name:"票据管理", items:[
                {name:"票据信息", url:"man_ticket_index"},
                {name:"出票管理", url:"man_pool_index"}
            ]},
            {name:"系统报表", items:[
                {name:"资产报表", url:"man_report_index"},
                {name:"返佣报表", url:"man_report_terminal.index"}
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
    new window.Com({id:self.main_id, path:"man_doc_index", pins:self}, cb);
}
