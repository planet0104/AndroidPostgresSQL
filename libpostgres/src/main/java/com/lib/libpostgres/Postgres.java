package com.lib.libpostgres;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Native;

public class Postgres {
    static final String TAG = Postgres.class.getSimpleName();

    public interface QueryCallback {
        void callback(String result, boolean success);
    }

    interface PostgresSys extends Library {

        public interface QueryCallbackSys extends Callback {
            void callback(String result, boolean success);
        }

        PostgresSys INSTANCE = (PostgresSys) Native.load("postgres", PostgresSys.class);

        void query(String host, int port, String user, String password, String dbname, String sql, QueryCallbackSys callback);
        void init_log();
    }

    public static void query(String host, int port, String user, String password, String dbname, String sql, QueryCallback callback){
        PostgresSys.INSTANCE.query(host, port, user, password, dbname, sql, new PostgresSys.QueryCallbackSys() {
            @Override
            public void callback(String result, boolean success) {
                callback.callback(result, success);
            }
        });
    }

    public static void initLog(){
        PostgresSys.INSTANCE.init_log();
    }
}
