package com.example.postgres;

import android.os.Bundle;
import android.util.Log;

import androidx.appcompat.app.AppCompatActivity;

import com.lib.libpostgres.Postgres;

public class MainActivity extends AppCompatActivity {
    static final String TAG = MainActivity.class.getSimpleName();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        //初始化日志
        Postgres.initLog();

        new Thread(new Runnable(){
            @Override
            public void run() {
                try {
                    Log.i(TAG,"开始链接数据库...");
                    Postgres.query("", 0, "", "", "test", "SELECT \"number\" FROM test", new Postgres.QueryCallback() {
                        @Override
                        public void callback(String result, boolean success) {
                            Log.i(TAG, "查询结果:"+ result+" success="+success);
                        }
                    });
                    Log.i(TAG,"sql执行结束.");
                } catch (Exception e) {
                    Log.e("MainActivity", "数据库查询失败:"+e.getMessage());
                    e.printStackTrace();
                }
            }
        }).start();
    }
}