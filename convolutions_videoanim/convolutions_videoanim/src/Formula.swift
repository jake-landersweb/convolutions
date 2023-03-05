//
//  Formula.swift
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/3/23.
//

import SwiftUI

struct Formula: View {
    
    @EnvironmentObject var model: Model
    
    var body: some View {
        VStack {
            Image("dft")
                .resizable()
                .scaledToFit()
                .padding(100)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

struct Formula_Previews: PreviewProvider {
    static var previews: some View {
        Formula()
    }
}
