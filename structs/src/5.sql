CREATE TABLE IF NOT EXISTS cosmos_#yyyy#_db.t_api_gateway_sens_log_#mmdd#  (
    -- 基础字段（顺序固定、一个不能少）
    Fid                    BIGINT(20)   NOT NULL AUTO_INCREMENT               COMMENT '主键',
    Fcreator               VARCHAR(32)  NOT NULL DEFAULT ''                    COMMENT '创建人',
    Fmodifier              VARCHAR(32)  NOT NULL DEFAULT ''                    COMMENT '修改人',
    Fcreate_time           DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP     COMMENT '创建时间',
    Fmodify_time           DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    Fis_deleted            TINYINT(4)   NOT NULL DEFAULT 0                     COMMENT '是否删除(0-否,1-是)',
    Fversion               INT(11)      NOT NULL DEFAULT 0                     COMMENT '版本号',

    -- 业务字段（按业务分组空行隔开）
    Faccess_time           DATETIME     NOT NULL DEFAULT '1970-01-01 00:00:00' COMMENT '访问时间',
    Fhost                  VARCHAR(256) NOT NULL DEFAULT ''                    COMMENT '域名',
    Fapi                   VARCHAR(256) NOT NULL DEFAULT ''                    COMMENT 'API路径',
    Fusr_tag_info          VARCHAR(512) NOT NULL DEFAULT ''                    COMMENT '账号标识 {x-s3-tid:x,mid:x,min:x,oa_session:x,oa_token_id:x,ip:x}',
    Freq_sens_info         TEXT         DEFAULT NULL                  COMMENT '入参敏感信息 [{敏感字段:value,敏感信息:{手机号:[],手机号数量:0,身份证:[]}}]',
    Frsp_sens_info         TEXT         DEFAULT NULL                  COMMENT '出参敏感信息 [{敏感字段:value,敏感信息:{手机号:[],手机号数量:0,身份证:[]}}]',
    Frsp_sens_fields_cnt   INT(11)      NOT NULL DEFAULT 0                     COMMENT '出参敏感字段数量',
    Ftask_start_time       DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP     COMMENT '任务开始时间',
    Ftask_end_time         DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP     COMMENT '任务结束时间',
    Faccount_match_status TINYINT(4)   NOT NULL DEFAULT 0                     COMMENT '账号匹配状态(0-待匹配,1-匹配成功,2-匹配失败)',
    Faccount_info          VARCHAR(128) NOT NULL DEFAULT ''                    COMMENT '账号信息',

    -- 索引定义（必须的 + 业务高频查询字段）
    PRIMARY KEY (Fid),
    KEY idx_Fmodify_time          USING BTREE (Fmodify_time),
    KEY idx_Faccess_time          USING BTREE (Faccess_time),
    KEY idx_Fhost                 USING BTREE (Fhost),
    KEY idx_Fapi                  USING BTREE (Fapi),
    KEY idx_Fusr_tag_info         USING BTREE (Fusr_tag_info),
    KEY idx_Faccount_match_status USING BTREE (Faccount_match_status),
    KEY idx_Faccount_info         USING BTREE (Faccount_info)

) ENGINE = InnoDB 
  DEFAULT CHARSET = utf8mb4 
  COMMENT = 'cosmos|API网关敏感信息日志表|t_api_gateway_sens_log|20251209';
