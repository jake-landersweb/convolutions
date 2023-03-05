//
//  convolutions_videoanimApp.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/3/23.
//

import SwiftUI

@main
struct convolutions_videoanimApp: App {
    @ObservedObject var model: Model = Model()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(model)
        }
    }
}
