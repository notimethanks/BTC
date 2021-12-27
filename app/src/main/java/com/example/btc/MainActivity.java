package com.example.btc;

import android.app.Activity;
import android.content.Context;
import android.graphics.Color;
import android.icu.text.DecimalFormat;
import android.os.Bundle;
import android.os.VibrationEffect;
import android.view.View;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.wear.widget.DismissibleFrameLayout;

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

import java.text.SimpleDateFormat;
import java.util.Calendar;
import java.util.Date;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;
import android.os.Vibrator;

public class MainActivity extends Activity{

    private ActivityMainBinding binding;
    private com.android.volley.toolbox.Volley Volley;

    //Market Data
    double priceclose = 0;
    double priceopen = 0;
    double marketCap = 0;
    //Block Data
    double blockcount = 0;
    int timesinceblock = 0;
    int blocktransactions = 0;
    //Lightning
    int lightningnodes = 0;


    String urlPrice = "https://api.gemini.com/v2/ticker/btcusd";
    String urlLightning ="https://1ml.com/statistics?json=true";
    String urlMempoolTxs="https://mempool.space/api/mempool";

    DecimalFormat df_usdFormatter = new DecimalFormat("#,###");
    DecimalFormat df_twodecimalsFormatter = new DecimalFormat("###.##");
    DecimalFormat df_onedecimalFormatter = new DecimalFormat("###.#");
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        setComplictionData();
    }
    //Main Display Data Loading
    public void setComplictionData(){
        TextView complicationtext_market = (TextView) findViewById(R.id.complicationtext_market);
        TextView complicationtext_blocks = (TextView) findViewById(R.id.complicationtext_blocks);
        TextView complicationtext_halving = (TextView) findViewById(R.id.complicationtext_halving);
        TextView complicationtext_mempool = (TextView) findViewById(R.id.complicationtext_mempool);
        TextView complicationtext_lightning = (TextView) findViewById(R.id.complicationtext_lightning);

        RequestQueue queue = Volley.newRequestQueue(this);

        //Fill mempool transactions complication

        JsonObjectRequest JsonRequestMempoolTransactions = new JsonObjectRequest
                (Request.Method.GET, urlMempoolTxs, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            complicationtext_mempool.setText(df_usdFormatter.format(response.getInt("count")));
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

        //Fill Blocks & Halving complications
        String url = "https://mempool.space/api/blocks";
        JsonArrayRequest JsonRequestBlocks = new JsonArrayRequest
                (Request.Method.GET, url, null, new Response.Listener<JSONArray>() {

                    @Override
                    public void onResponse(JSONArray response) {
                        try {
                            JSONObject block = response.getJSONObject(0);
                            int blockheight = block.getInt("height");
                            complicationtext_blocks.setText(String.valueOf(blockheight/1000)+"k");
                            complicationtext_halving.setText(String.valueOf((840000-blockheight)/144)+"d");
                        } catch (JSONException e) {
                            e.printStackTrace();
                        }
                    }
                }, new Response.ErrorListener() {

                    @Override
                    public void onErrorResponse(VolleyError error) {

                    }
                });


        //Fill Market complication

        JsonObjectRequest JsonRequestPrice = new JsonObjectRequest
                (Request.Method.GET, urlPrice, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            priceclose = Double.parseDouble(response.getString("close"));
                            complicationtext_market.setText("$"+df_onedecimalFormatter.format(priceclose/1000)+"k");

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
        //update lightning complication
        JsonObjectRequest JsonRequestLightningNodes = new JsonObjectRequest
                (Request.Method.GET, urlLightning, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            lightningnodes= response.getInt("numberofnodes");
                            lightningnodes= lightningnodes/1000;
                            complicationtext_lightning.setText(String.valueOf(lightningnodes)+"k");
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
        //Add request every 30 seconds.
        Runnable myRepeater = new Runnable() {
            public void run() {
                queue.add(JsonRequestPrice);
                queue.add(JsonRequestBlocks);
                queue.add(JsonRequestLightningNodes);
                queue.add(JsonRequestMempoolTransactions);
            }
        };
        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);
        executor.scheduleAtFixedRate(myRepeater, 0, 30, TimeUnit.SECONDS);
    }

    public void FIllMarketLayout() {
        TextView textViewPrice = (TextView) findViewById(R.id.tv_price);
        TextView textViewPriceChange = (TextView) findViewById(R.id.tv_pricechange24);
        ImageView exrateicon = (ImageView) findViewById(R.id.image_bitcoinlogo);
        RequestQueue queue = Volley.newRequestQueue(this);
        JsonObjectRequest JsonObjectRequestPriceClose = new JsonObjectRequest
                (Request.Method.GET, urlPrice, null, new Response.Listener<JSONObject>() {

                    @Override
                    public void onResponse(JSONObject response) {
                        try {
                            priceclose = Double.parseDouble(response.getString("close"));
                            textViewPrice.setText("$" + df_usdFormatter.format(priceclose));

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
                            textViewPriceChange.setText(df_twodecimalsFormatter.format(((priceclose/priceopen)-1)*100)+"%");
                            if(((priceclose/priceopen)-1)*100>0){
                                textViewPriceChange.setTextColor(Color.parseColor("#71cc71"));
                                exrateicon.setImageDrawable(getDrawable(R.drawable.drawable_upmarket_green));
                                exrateicon.setRotation(0);
                                textViewPriceChange.setText("+"+textViewPriceChange.getText());
                            }
                            else{
                                textViewPriceChange.setTextColor(Color.parseColor("#cc7171"));
                                exrateicon.setImageDrawable(getDrawable(R.drawable.drawable_downmarket_red));
                                exrateicon.setRotation(0);
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
        String url = "https://blockchain.info/q/marketcap";
        TextView marketcaptextview = (TextView) findViewById(R.id.tv_marketcap);
        StringRequest StringRequestMarketcap = new StringRequest(Request.Method.GET, url,
                new Response.Listener<String>() {
                    @Override
                    public void onResponse(String response) {
                        marketCap=Double.parseDouble(response);
                        marketCap=marketCap/1000000000;
                        Math.round(marketCap);

                        marketcaptextview.setText("$"+df_usdFormatter.format(marketCap)+"B Market Cap");

                    }
                },
                new Response.ErrorListener() {
                    @Override
                    public void onErrorResponse(VolleyError error) {
                        // Handle error
                    }
                });

        //Add the request.
        queue.add(JsonObjectRequestPriceClose);
        queue.add(JsonObjectRequestPriceOpen);
        queue.add(StringRequestMarketcap);


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

    public void FillHalvingLayout(){
        String url = "https://mempool.space/api/blocks";
        TextView daysToHalving = (TextView) findViewById(R.id.tv_halvingdays);
        TextView blocksToHalving = (TextView) findViewById(R.id.tv_blockstohalving);
        TextView halvingETA = (TextView) findViewById(R.id.tv_halvingeta);
        RequestQueue queue = Volley.newRequestQueue(this);
        JsonArrayRequest JsonRequestBlocks = new JsonArrayRequest
                (Request.Method.GET, url, null, new Response.Listener<JSONArray>() {

                    @Override
                    public void onResponse(JSONArray response) {
                        try {
                            JSONObject block = response.getJSONObject(0);
                            int blockheight = block.getInt("height");
                            daysToHalving.setText(df_usdFormatter.format(840000-blockheight));
                            blocksToHalving.setText("~ "+String.valueOf((840000-blockheight)/144)+" Days");
                            SimpleDateFormat sdf = new SimpleDateFormat("MMMM dd yyyy");
                            Calendar c = Calendar.getInstance();
                            c.setTime(new Date()); // Using today's date
                            c.add(Calendar.DATE, (840000-blockheight)/144); // Adding 5 days
                            String output = sdf.format(c.getTime());
                            halvingETA.setText("~ "+output);
                        } catch (JSONException e) {
                            e.printStackTrace();
                        }
                    }
                }, new Response.ErrorListener() {

                    @Override
                    public void onErrorResponse(VolleyError error) {

                    }
                });
        queue.add(JsonRequestBlocks);
        //Add request every 30 seconds.
        Runnable myRepeater = new Runnable() {
            public void run() {
                queue.add(JsonRequestBlocks);
            }
        };

        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);
        executor.scheduleAtFixedRate(myRepeater, 0, 30, TimeUnit.SECONDS);

    }

    public void FillBlocksLayout(){
        String url = "https://mempool.space/api/blocks";
        RequestQueue queue = Volley.newRequestQueue(this);
        TextView blockcounttextview = (TextView) findViewById(R.id.tv_blockcount);
        TextView tvtx = (TextView) findViewById(R.id.tv_blocktransactions);
        TextView tvtime = (TextView) findViewById(R.id.tv_timesinceblock);
        JsonArrayRequest jsonArrayRequest = new JsonArrayRequest
                (Request.Method.GET, url, null, new Response.Listener<JSONArray>() {


                    @Override
                    public void onResponse(JSONArray response) {
                        try {
                            JSONObject block = response.getJSONObject(0);
                            int timestamp = block.getInt("timestamp");
                            blocktransactions=block.getInt("tx_count");
                            blockcount=block.getInt("height");
                            Date datenow = new Date();
                            blockcounttextview.setText(df_usdFormatter.format(blockcount));
                            timesinceblock= (int) ((datenow.getTime()/1000)-timestamp);
                            timesinceblock=timesinceblock/60;
                            tvtx.setText(df_usdFormatter.format(blocktransactions)+" Transactions");
                            tvtime.setText("Mined "+timesinceblock+" minutes ago");
                            if(timesinceblock>25){
                                tvtime.setTextColor(Color.parseColor("#cc7171"));
                            }
                            else{
                                tvtime.setTextColor(Color.parseColor("#D5D5D5"));
                            }
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


//Market Complication Clicked
    public void openMarketLayout(View view) {
        Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
        setContentView(R.layout.activity_market);
        DismissibleFrameLayout mylayout = (DismissibleFrameLayout) findViewById(R.id.market_frame);
        mylayout.registerCallback(new DismissibleFrameLayout.Callback() {
                                      @Override
                                      public void onDismissFinished(@NonNull DismissibleFrameLayout layout) {
                                          super.onDismissFinished(layout);
                                          Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
                                          v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
                                          setContentView(R.layout.activity_main);
                                          setComplictionData();
                                      }
                                  });
                FIllMarketLayout();
    }
//Blocks Complication Clicked
    public void onMenuBlocks(View view) {
        Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
        setContentView(R.layout.activity_blocks);
        DismissibleFrameLayout mylayout = (DismissibleFrameLayout) findViewById(R.id.blocks_frame);
        mylayout.registerCallback(new DismissibleFrameLayout.Callback() {
            @Override
            public void onDismissFinished(@NonNull DismissibleFrameLayout layout) {
                super.onDismissFinished(layout);
                Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
                v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
                setContentView(R.layout.activity_main);
                setComplictionData();
            }
        });
        FillBlocksLayout();
    }
//Halving Complication Clicked
    public void onMenuHalving(View view) {
        Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
        v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
        setContentView(R.layout.activity_halving);
        DismissibleFrameLayout mylayout = (DismissibleFrameLayout) findViewById(R.id.halving_frame);
        mylayout.registerCallback(new DismissibleFrameLayout.Callback() {
            @Override
            public void onDismissFinished(@NonNull DismissibleFrameLayout layout) {
                super.onDismissFinished(layout);
                Vibrator v = (Vibrator) getSystemService(Context.VIBRATOR_SERVICE);
                v.vibrate(VibrationEffect.createOneShot(15, VibrationEffect.DEFAULT_AMPLITUDE));
                setContentView(R.layout.activity_main);
                setComplictionData();
            }
        });
        FillHalvingLayout();
    }



}

