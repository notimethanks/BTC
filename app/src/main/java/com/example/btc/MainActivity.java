package com.example.btc;

import android.app.Activity;
import android.graphics.Color;
import android.icu.text.DecimalFormat;
import android.os.Bundle;
import android.widget.TextView;

import com.android.volley.Request;
import com.android.volley.RequestQueue;
import com.android.volley.Response;
import com.android.volley.VolleyError;
import com.android.volley.toolbox.JsonArrayRequest;
import com.android.volley.toolbox.JsonObjectRequest;
import com.android.volley.toolbox.StringRequest;
import com.example.btc.databinding.ActivityMainBinding;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import java.util.Date;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class MainActivity extends Activity {

    private ActivityMainBinding binding;
    private com.android.volley.toolbox.Volley Volley;
    //market data
    double priceclose = 0;
    double priceopen = 0;
    double marketCap = 0;
    double blockcount = 0;
    int timesinceblock = 0;
    int blocktransactions = 0;


    String urlPrice = "https://api.gemini.com/v2/ticker/btcusd";

    DecimalFormat usdFormatter = new DecimalFormat("#,###");
    DecimalFormat twodecimalsFormatter = new DecimalFormat("###.##");
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        getMarketCap();
        getAndSetPriceData();
        getBlockCount();
        getBlocks();


    }

    public void getAndSetPriceData() {
        TextView textViewPrice = (TextView) findViewById(R.id.tv_price);
        TextView textViewPriceChange = (TextView) findViewById(R.id.tv_pricechange24);
        RequestQueue queue = Volley.newRequestQueue(this);
        JsonObjectRequest JsonObjectRequestPriceClose = new JsonObjectRequest
                (Request.Method.GET, urlPrice, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            priceclose = Double.parseDouble(response.getString("close"));
                            textViewPrice.setText("$" + usdFormatter.format(priceclose));

                        } catch (JSONException e) {
                            e.printStackTrace();
                        }
                    }
                }, new Response.ErrorListener() {

                    @Override
                    public void onErrorResponse(VolleyError error) {
                        // TODO: Handle error

                    }
                });
        JsonObjectRequest JsonObjectRequestPriceOpen = new JsonObjectRequest
                (Request.Method.GET, urlPrice, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            priceopen = Double.parseDouble(response.getString("open"));
                            textViewPriceChange.setText(twodecimalsFormatter.format(((priceclose/priceopen)-1)*100)+"%");
                            if(((priceclose/priceopen)-1)*100>0){
                                textViewPriceChange.setTextColor(Color.parseColor("#FF66BB6A"));
                                textViewPriceChange.setText("+"+textViewPriceChange.getText());
                            }
                            else{
                                textViewPriceChange.setTextColor(Color.parseColor("#FFEF5350"));
                            }

                        } catch (JSONException e) {
                            e.printStackTrace();
                        }
                    }
                }, new Response.ErrorListener() {

                    @Override
                    public void onErrorResponse(VolleyError error) {
                        // TODO: Handle error

                    }
                });

        //Add the request.
        queue.add(JsonObjectRequestPriceClose);
        queue.add(JsonObjectRequestPriceOpen);

        //Add request every 3 seconds.
        Runnable myRepeater = new Runnable() {
            public void run() {
                queue.add(JsonObjectRequestPriceClose);
                queue.add(JsonObjectRequestPriceOpen);
            }
        };

        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);
        executor.scheduleAtFixedRate(myRepeater, 0, 3, TimeUnit.SECONDS);
    }

    public void getMarketCap(){
        String url = "https://blockchain.info/q/marketcap";
        TextView marketcaptextview = (TextView) findViewById(R.id.tv_marketcap);
        RequestQueue queue = Volley.newRequestQueue(this);
        StringRequest stringRequest = new StringRequest(Request.Method.GET, url,
                new Response.Listener<String>() {
                    @Override
                    public void onResponse(String response) {
                        marketCap=Double.parseDouble(response);
                        marketCap=marketCap/1000000000;
                        Math.round(marketCap);

                        marketcaptextview.setText("$"+usdFormatter.format(marketCap)+"B Market Cap");

                    }
                },
                new Response.ErrorListener() {
                    @Override
                    public void onErrorResponse(VolleyError error) {
                        // Handle error
                    }
                });

        queue.add(stringRequest);

    }

    public void getBlockCount(){
        String url = "https://blockchain.info/q/getblockcount";
        TextView blockcounttextview = (TextView) findViewById(R.id.tv_blockcount);
        RequestQueue queue = Volley.newRequestQueue(this);
        StringRequest stringRequest = new StringRequest(Request.Method.GET, url,
                new Response.Listener<String>() {
                    @Override
                    public void onResponse(String response) {
                        blockcount=Double.parseDouble(response);
                        blockcounttextview.setText(usdFormatter.format(blockcount));

                    }
                },
                new Response.ErrorListener() {
                    @Override
                    public void onErrorResponse(VolleyError error) {
                        // Handle error
                    }
                });

        queue.add(stringRequest);

        //Add request every 30 seconds.
        Runnable myRepeater = new Runnable() {
            public void run() {
                queue.add(stringRequest);
            }
        };

        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);
        executor.scheduleAtFixedRate(myRepeater, 0, 30, TimeUnit.SECONDS);

    }

    public void getBlocks(){
        String url = "https://mempool.space/api/blocks";
        RequestQueue queue = Volley.newRequestQueue(this);
        JsonArrayRequest jsonArrayRequest = new JsonArrayRequest
                (Request.Method.GET, url, null, new Response.Listener<JSONArray>() {


                    @Override
                    public void onResponse(JSONArray response) {
                        try {
                            JSONObject block = response.getJSONObject(0);
                            int timestamp = block.getInt("timestamp");
                            blocktransactions=block.getInt("tx_count");
                            Date datenow = new Date();
                            timesinceblock= (int) ((datenow.getTime()/1000)-timestamp);
                            timesinceblock=timesinceblock/60;
                            TextView tvtx = (TextView) findViewById(R.id.tv_blocktransactions);
                            tvtx.setText(usdFormatter.format(blocktransactions)+" Transactions");
                            TextView tvtime = (TextView) findViewById(R.id.tv_timesinceblock);
                            tvtime.setText("Mined "+timesinceblock+" minutes ago");
                            if(timesinceblock==0){
                                tvtime.setText("Mined just now");
                            }
                            else if(timesinceblock==1){
                                tvtime.setText("Mined a minute ago");
                            }
                        } catch (JSONException e) {
                            e.printStackTrace();
                        }
                    }
                }, new Response.ErrorListener() {

                    @Override
                    public void onErrorResponse(VolleyError error) {

                    }
                });
        queue.add(jsonArrayRequest);
        //Add request every 30 seconds.
        Runnable myRepeater = new Runnable() {
            public void run() {
                queue.add(jsonArrayRequest);
            }
        };

        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);
        executor.scheduleAtFixedRate(myRepeater, 0, 30, TimeUnit.SECONDS);
    }






}

