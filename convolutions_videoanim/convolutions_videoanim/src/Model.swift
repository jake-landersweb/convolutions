//
//  Model.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/3/23.
//

import Foundation
import SwiftUI

// constants
let accColor: Color = Color(red: 53 / 255, green: 58.2 / 255, blue: 183.9 / 255)
let bgColor: Color = Color(red: 21.7/255, green: 28.2/255, blue: 38.9/255)
let anim = Animation.spring()

class Model: ObservableObject {
    init() {
        Task.init {
            await run()
        }
    }
    
    @Published var show = false
    
    func run() async {
        // sleep for 5 seconds at start
        try? await Task.sleep(nanoseconds: 1_000_000_000)
        
        DispatchQueue.main.async {
            withAnimation(anim) {
                self.show = !self.show
            }
        }
    }
}
