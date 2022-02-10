package lt.ekgame.bancho.client;

import lt.ekgame.bancho.api.exceptions.LoginException;
import lt.ekgame.bancho.api.packets.server.PacketReceivingFinished;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        try {
            // Create and instance with the username and password that the bot will use
            BanchoClient banchoClient = new BanchoClient("-Spring Night-", "z3xyht7wl9", false, false);
            // Register a handeler to take action on some kind of an event
            banchoClient.registerHandler((packet -> {
                switch (packet){
                    case PacketReceivingFinished e ->{
                        System.out.println("Welcome to osu!Bancho!");
//                        banchoClient.sendMessage("Krismile","哼哼,我上线了");
                    }
                    default -> {}
                }
//                System.out.println(packet.getClass().getName());
            }));
            // Authenticate the account (sync)
            banchoClient.connect();

            // Start the client (async)
            banchoClient.start();
            new Thread(()->{
                Scanner sc = new Scanner(System.in);
                while (true){
                    banchoClient.sendMessage("Krismile",sc.next());
                }
            }).start();

        } catch (URISyntaxException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        } catch (LoginException e) {
            e.printStackTrace();
        }
    }
}
